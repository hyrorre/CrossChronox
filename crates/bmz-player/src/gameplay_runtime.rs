//! Gameplay owns mutable session state. The window thread sends commands and
//! consumes detached observations; it never holds a gameplay lock while drawing.
use std::collections::VecDeque;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc,
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::screens::{
    play_finish::FinishSessionSnapshot,
    play_session::AppliedArrange,
    play_snapshot::{BgaFrameCatalog, PlayRenderSnapshotCache},
    result_model::ResultGraphCollector,
};
use anyhow::{Result, anyhow};
use bmz_audio::command::AudioEngineHandle;
use bmz_core::time::TimeUs;
use bmz_gameplay::runtime::GameplayRuntime;
use bmz_gameplay::session::{FrameOutput, GameSession, PlayState, SkinRuntimeEvent};
use bmz_render::snapshot::RenderSnapshot;

mod observation;
pub use observation::PlaySessionObservation;

static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);
const COMMAND_CAPACITY: usize = 64;
const PRESENTATION_HISTORY_CAPACITY: usize = 8192;
const SAFETY_WAKE: Duration = Duration::from_millis(2);

pub struct RuntimeRenderConfig {
    pub best_ex_score: Option<u32>,
    pub best_ghost: Option<Vec<u8>>,
    pub target_ex_score: Option<u32>,
    pub target: String,
    pub applied_arrange: AppliedArrange,
    pub source_ln_profile: crate::ln_policy::ChartLnProfile,
    pub skin_attempt: bmz_render::snapshot::SkinAttemptState,
    pub score_key: crate::storage::score_db::ScoreKey,
    pub practice_mode: bool,
    pub score_save_disabled: bool,
    pub bga_frames: BgaFrameCatalog,
    pub cache: PlayRenderSnapshotCache,
}

#[derive(Clone)]
pub struct RuntimeResult {
    pub snapshot: FinishSessionSnapshot,
    pub graph: ResultGraphCollector,
    pub settled_at: TimeUs,
    pub play_duration_ms: u64,
}

struct Publication {
    generation: u64,
    session: PlaySessionObservation,
    frame: FrameOutput<RenderSnapshot>,
    result: Option<Arc<RuntimeResult>>,
}

type EditSession = Box<dyn FnOnce(&mut GameSession) + Send>;
struct Worker {
    commands: mpsc::SyncSender<EditSession>,
    latest: Arc<Mutex<Option<Publication>>>,
    stop: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

/// Prepared sessions stay local until READY. During play this is a command
/// endpoint plus immutable observations, with no access to the live GameSession.
pub struct GameplayClient {
    pub session: PlaySessionObservation,
    local: Option<GameplayRuntime>,
    worker: Option<Worker>,
    generation: u64,
    latest_frame: Option<FrameOutput<RenderSnapshot>>,
    pub result: Option<Arc<RuntimeResult>>,
}

impl GameplayClient {
    pub fn new(session: GameSession) -> Self {
        Self {
            session: PlaySessionObservation::from_session(&session),
            local: Some(GameplayRuntime::new(session)),
            worker: None,
            generation: NEXT_GENERATION.fetch_add(1, Ordering::Relaxed),
            latest_frame: None,
            result: None,
        }
    }

    pub fn prepared_session(&self) -> Option<&GameSession> {
        self.local.as_ref().map(|runtime| &runtime.session)
    }

    pub fn configure_prepared<R>(&mut self, edit: impl FnOnce(&mut GameSession) -> R) -> R {
        let runtime =
            self.local.as_mut().expect("prepared session cannot be edited after runtime start");
        let result = edit(&mut runtime.session);
        self.session = PlaySessionObservation::from_session(&runtime.session);
        result
    }

    pub fn edit(&mut self, edit: impl FnOnce(&mut GameSession) + Send + 'static) -> bool {
        if let Some(runtime) = &mut self.local {
            edit(&mut runtime.session);
            self.session = PlaySessionObservation::from_session(&runtime.session);
            return true;
        }
        let Some(worker) = &self.worker else {
            return false;
        };
        match worker.commands.try_send(Box::new(edit)) {
            Ok(()) => {
                worker.thread.thread().unpark();
                true
            }
            Err(_) => {
                tracing::error!(generation = self.generation, "gameplay control queue unavailable");
                false
            }
        }
    }

    pub fn start(&mut self, audio: AudioEngineHandle, config: RuntimeRenderConfig) -> Result<()> {
        if self.worker.is_some() {
            return Ok(());
        }
        let runtime = self.local.take().ok_or_else(|| anyhow!("gameplay is not prepared"))?;
        let times = bmz_gameplay::session::compute_frame_times(&runtime.session);
        self.latest_frame =
            Some(crate::screens::play_loop::frame_output_from_session_frame_cached(
                &runtime.session,
                bmz_gameplay::session::SessionFrame {
                    times,
                    judgements: Vec::new(),
                    mine_hits: Vec::new(),
                    keysound_volumes: Vec::new(),
                    skin_events: Vec::new(),
                    state: runtime.session.state,
                },
                config.best_ex_score,
                config.best_ghost.as_deref(),
                config.target_ex_score,
                &config.bga_frames,
                &config.cache,
            ));
        let latest = Arc::new(Mutex::new(None));
        let stop = Arc::new(AtomicBool::new(false));
        let (commands, receiver) = mpsc::sync_channel(COMMAND_CAPACITY);
        let worker_latest = Arc::clone(&latest);
        let worker_stop = Arc::clone(&stop);
        let generation = self.generation;
        let thread =
            thread::Builder::new().name(format!("bmz-gameplay-{generation}")).spawn(move || {
                run(runtime, audio, config, receiver, worker_latest, worker_stop, generation)
            })?;
        self.worker = Some(Worker { commands, latest, stop, thread });
        tracing::info!(
            generation,
            "gameplay runtime: dedicated thread; audio scheduling: gameplay runtime"
        );
        Ok(())
    }

    pub fn poll(&mut self) -> Option<FrameOutput<RenderSnapshot>> {
        let worker = self.worker.as_ref()?;
        let publication = worker.latest.try_lock().ok().and_then(|mut latest| latest.take());
        if let Some(publication) = publication {
            if publication.generation != self.generation {
                return None;
            }
            self.session = publication.session;
            self.result = publication.result;
            self.latest_frame = Some(publication.frame);
        }
        self.latest_frame.clone()
    }

    pub fn is_running(&self) -> bool {
        self.worker.is_some()
    }

    pub fn shutdown(&mut self) {
        if let Some(worker) = self.worker.take() {
            worker.stop.store(true, Ordering::Release);
            worker.thread.thread().unpark();
            // No device calls or renderer work runs in this thread. Never wait
            // on it in the window callback; its owned session is retired there.
            if worker.thread.is_finished() {
                let _ = worker.thread.join();
            }
        }
    }
}

impl Drop for GameplayClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn run(
    mut runtime: GameplayRuntime,
    audio: AudioEngineHandle,
    config: RuntimeRenderConfig,
    commands: mpsc::Receiver<EditSession>,
    latest: Arc<Mutex<Option<Publication>>>,
    stop: Arc<AtomicBool>,
    generation: u64,
) {
    let mut graph = ResultGraphCollector::default();
    let mut history = VecDeque::<SkinRuntimeEvent>::with_capacity(PRESENTATION_HISTORY_CAPACITY);
    let mut result = None;
    let mut terminal_result = false;
    let mut last_time = TimeUs(i64::MIN);
    runtime.session.input_system.backend.set_waker(Some(thread::current()));
    while !stop.load(Ordering::Acquire) {
        let iteration_started = Instant::now();
        for edit in commands.try_iter() {
            edit(&mut runtime.session);
        }
        if stop.load(Ordering::Acquire) {
            break;
        }
        let frame = runtime.advance(&audio);
        let mut frame = crate::screens::play_loop::frame_output_from_session_frame_cached(
            &runtime.session,
            frame,
            config.best_ex_score,
            config.best_ghost.as_deref(),
            config.target_ex_score,
            &config.bga_frames,
            &config.cache,
        );
        let snapshot = &mut frame.render_snapshot;
        crate::screens::play_loop::apply_play_arrange_to_snapshot(
            snapshot,
            &config.applied_arrange,
        );
        snapshot.target.clone_from(&config.target);
        snapshot.skin_attempt = config.skin_attempt;
        snapshot.rule_mode_index =
            crate::skin_extension::rule_mode_index(config.score_key.rule_mode);
        snapshot.ln_score_policy_index =
            Some(crate::skin_extension::ln_score_policy_index(config.score_key.ln_policy));
        snapshot.practice_mode = config.practice_mode;
        snapshot.score_save_enabled = !snapshot.autoplay
            && !snapshot.replay_playback
            && !config.practice_mode
            && !config.score_save_disabled
            && runtime.session.assist.score_update_enabled();
        crate::screens::play_snapshot::refresh_play_skin_visuals(snapshot, &runtime.session);
        debug_assert!(
            snapshot.time >= last_time,
            "gameplay clock moved backwards within a generation"
        );
        last_time = snapshot.time;
        graph.record_frame(&frame);
        let terminal = matches!(runtime.session.state, PlayState::Finished | PlayState::Failed);
        if (result.is_none()
            && bmz_gameplay::session::result_is_settled(&runtime.session, last_time))
            || (terminal && !terminal_result)
        {
            result = Some(Arc::new(RuntimeResult {
                snapshot: FinishSessionSnapshot::from_session(
                    &runtime.session,
                    config.source_ln_profile,
                    &config.applied_arrange,
                ),
                graph: graph.clone(),
                settled_at: last_time,
                play_duration_ms: (runtime.session.audio_clock.elapsed_since(TimeUs(0)).0.max(0)
                    / 1000) as u64,
            }));
            terminal_result = terminal;
        }
        for event in frame.skin_events.drain(..) {
            if history.len() == PRESENTATION_HISTORY_CAPACITY {
                history.pop_front();
            }
            history.push_back(event);
        }
        frame.render_snapshot.skin_events = history.iter().cloned().collect();
        let publication = Publication {
            generation,
            session: PlaySessionObservation::from_session(&runtime.session),
            frame,
            result: result.clone(),
        };
        // The lock protects only the pointer exchange, never computation or GPU
        // work. try_lock on both ends means neither thread waits for the other.
        let old =
            if let Ok(mut slot) = latest.try_lock() { slot.replace(publication) } else { None };
        drop(old);
        let remaining =
            runtime.next_wake_after(SAFETY_WAKE).saturating_sub(iteration_started.elapsed());
        if !remaining.is_zero() {
            thread::park_timeout(remaining);
        }
    }
}
