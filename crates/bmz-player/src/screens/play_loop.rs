#[cfg(test)]
use std::sync::TryLockError;

use anyhow::{Result, anyhow};
use bmz_audio::backend::cpal::SharedAudioEngine;
use bmz_audio::command::AudioEngineHandle;
use bmz_audio::queue::{AudioScheduler, ScheduledSoundQueue};
use bmz_core::{ids::SoundId, time::TimeUs};
use bmz_gameplay::session::{
    FrameOutput, GameSession, PlayState, SessionFrame, advance_session_frame,
    apply_auto_key_release, compute_frame_times, update_recent_inputs, update_recent_judgements,
};
use bmz_render::snapshot::RenderSnapshot;

use crate::audio::RunningPlaySession;
use crate::config::profile_config::{IrConfig, ReplayConfig};
use crate::paths::ProfilePaths;
use crate::screens::play_finish::{
    FinishResultMode, FinishSessionResultOnceRequest, FinishSessionResultRequest,
    FinishedPlaySession, finish_session_result, finish_session_result_once,
};
use crate::screens::play_session::AppliedArrange;
use crate::screens::play_snapshot::{
    BgaFrameCatalog, PlayRenderSnapshotCache, build_render_snapshot_with_target_and_bga_frames,
    build_render_snapshot_with_target_and_bga_frames_cached,
};
use crate::storage::network_db::NetworkDatabase;
use crate::storage::score_db::ScoreDatabase;

#[derive(Debug, Clone)]
pub enum PlayAdvanceOutcome {
    Playing(FrameOutput<RenderSnapshot>),
    Finished { frame: FrameOutput<RenderSnapshot>, finished: Box<FinishedPlaySession> },
}

impl PlayAdvanceOutcome {
    pub fn frame(&self) -> &FrameOutput<RenderSnapshot> {
        match self {
            Self::Playing(frame) | Self::Finished { frame, .. } => frame,
        }
    }

    pub fn finished(&self) -> Option<&FinishedPlaySession> {
        match self {
            Self::Playing(_) => None,
            Self::Finished { finished, .. } => Some(finished),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished().is_some()
    }
}

pub fn advance_play_screen(
    session: &mut GameSession,
    audio: &mut dyn AudioScheduler,
    best_ex_score: Option<u32>,
) -> FrameOutput<RenderSnapshot> {
    advance_play_screen_with_bga_frames(
        session,
        audio,
        best_ex_score,
        None,
        None,
        &BgaFrameCatalog::new(),
    )
}

pub fn advance_play_screen_with_bga_frames(
    session: &mut GameSession,
    audio: &mut dyn AudioScheduler,
    best_ex_score: Option<u32>,
    best_ghost: Option<&[u8]>,
    target_ex_score: Option<u32>,
    bga_frames: &BgaFrameCatalog,
) -> FrameOutput<RenderSnapshot> {
    let frame = advance_session_frame(session, audio);
    let mut render_snapshot = build_render_snapshot_with_target_and_bga_frames(
        session,
        frame.times.audio_now,
        &session.recent_judgements,
        best_ex_score,
        best_ghost,
        target_ex_score,
        bga_frames,
    );
    render_snapshot.skin_events = frame.skin_events.clone();
    FrameOutput {
        render_snapshot,
        judgements: frame.judgements,
        mine_hits: frame.mine_hits,
        keysound_volumes: frame.keysound_volumes,
        skin_events: frame.skin_events,
        state: frame.state,
    }
}

pub fn advance_play_screen_until_result(
    session: &mut GameSession,
    audio: &mut dyn AudioScheduler,
    score_db: &mut ScoreDatabase,
    network_db: &mut NetworkDatabase,
    profile_paths: &ProfilePaths,
    replay_config: &ReplayConfig,
    ir_config: &IrConfig,
    played_at: i64,
    applied_arrange: &AppliedArrange,
) -> Result<PlayAdvanceOutcome> {
    let frame = advance_play_screen(session, audio, None);
    if matches!(frame.state, PlayState::Finished | PlayState::Failed) {
        let mut finished = finish_session_result(
            score_db,
            network_db,
            FinishSessionResultRequest {
                profile_paths,
                replay_config,
                ir_config,
                session,
                played_at,
                applied_arrange,
                source_ln_profile: crate::ln_policy::ChartLnProfile::from_chart(&session.chart),
                chart_length_ms: None,
                play_duration_ms: None,
                target_ex_score: None,
                score_key: crate::storage::score_db::ScoreKey::new(
                    session.chart.identity.file_sha256,
                    crate::ln_policy::score_ln_policy(
                        crate::ln_policy::LnPolicySetting::AutoLn,
                        crate::ln_policy::ChartLnProfile::from_chart(&session.chart),
                    ),
                )
                .with_rule_mode(session.rule_mode),
                practice_mode: false,
                finish_mode: FinishResultMode::Normal,
            },
        )?;
        let mut result_graph = crate::screens::result_model::ResultGraphCollector::default();
        result_graph.record_frame(&frame);
        finished.summary.graph = std::sync::Arc::new(result_graph.snapshot_for_session(session));
        return Ok(PlayAdvanceOutcome::Finished { frame, finished: Box::new(finished) });
    }

    Ok(PlayAdvanceOutcome::Playing(frame))
}

pub fn advance_play_screen_with_shared_audio(
    session: &mut GameSession,
    audio: &SharedAudioEngine,
    best_ex_score: Option<u32>,
) -> Result<FrameOutput<RenderSnapshot>> {
    let mut scheduled = ScheduledSoundQueue::new();
    let frame = advance_session_frame(session, &mut scheduled);
    flush_scheduled_audio_blocking(audio, &mut scheduled)?;
    Ok(frame_output_from_session_frame(
        session,
        frame,
        best_ex_score,
        None,
        None,
        &BgaFrameCatalog::new(),
    ))
}

/// `SessionFrame`(audio スケジューリング結果)から、ロック不要な render
/// snapshot を構築して `FrameOutput` を組み立てる。重い処理はここに集約し、
/// audio エンジンロックの外で実行する。
fn frame_output_from_session_frame(
    session: &GameSession,
    frame: SessionFrame,
    best_ex_score: Option<u32>,
    best_ghost: Option<&[u8]>,
    target_ex_score: Option<u32>,
    bga_frames: &BgaFrameCatalog,
) -> FrameOutput<RenderSnapshot> {
    let cache = PlayRenderSnapshotCache::from_chart(&session.chart);
    frame_output_from_session_frame_cached(
        session,
        frame,
        best_ex_score,
        best_ghost,
        target_ex_score,
        bga_frames,
        &cache,
    )
}

fn frame_output_from_session_frame_cached(
    session: &GameSession,
    frame: SessionFrame,
    best_ex_score: Option<u32>,
    best_ghost: Option<&[u8]>,
    target_ex_score: Option<u32>,
    bga_frames: &BgaFrameCatalog,
    cache: &PlayRenderSnapshotCache,
) -> FrameOutput<RenderSnapshot> {
    let mut render_snapshot = build_render_snapshot_with_target_and_bga_frames_cached(
        session,
        frame.times.audio_now,
        &session.recent_judgements,
        best_ex_score,
        best_ghost,
        target_ex_score,
        bga_frames,
        cache,
    );
    render_snapshot.play_elapsed_time = TimeUs(frame.times.audio_now.0.max(0));
    render_snapshot.skin_events = frame.skin_events.clone();
    FrameOutput {
        render_snapshot,
        judgements: frame.judgements,
        mine_hits: frame.mine_hits,
        keysound_volumes: frame.keysound_volumes,
        skin_events: frame.skin_events,
        state: frame.state,
    }
}

fn flush_scheduled_audio_blocking(
    audio: &SharedAudioEngine,
    scheduled: &mut ScheduledSoundQueue,
) -> Result<()> {
    if scheduled.is_empty() {
        return Ok(());
    }
    let mut audio = audio.lock().map_err(|_| anyhow!("audio engine lock poisoned"))?;
    audio.schedule_all(scheduled.drain_all());
    Ok(())
}

#[cfg(test)]
fn flush_scheduled_audio_nonblocking(
    audio: &SharedAudioEngine,
    scheduled: &mut ScheduledSoundQueue,
) -> Result<()> {
    if scheduled.is_empty() {
        return Ok(());
    }
    match audio.try_lock() {
        Ok(mut audio) => {
            audio.schedule_all(scheduled.drain_all());
            Ok(())
        }
        Err(TryLockError::WouldBlock) => Ok(()),
        Err(TryLockError::Poisoned(_)) => Err(anyhow!("audio engine lock poisoned")),
    }
}

#[cfg(test)]
fn flush_scheduled_audio_commands(
    audio: &AudioEngineHandle,
    scheduled: &mut ScheduledSoundQueue,
) -> Result<()> {
    log_audio_scheduling_latency(audio);
    if scheduled.is_empty() {
        return Ok(());
    }
    let sounds = scheduled.drain_all().collect::<Vec<_>>();
    match audio.try_schedule_all(sounds) {
        Ok(()) => {
            bmz_gameplay::session::latency::audio_enqueued();
            Ok(())
        }
        Err(sounds) => {
            for sound in sounds {
                scheduled.schedule(sound);
            }
            Ok(())
        }
    }
}

fn log_audio_scheduling_latency(audio: &AudioEngineHandle) {
    if !tracing::enabled!(tracing::Level::DEBUG) {
        return;
    }
    thread_local! {
        static LAST_LOG: std::cell::Cell<Option<std::time::Instant>> = const { std::cell::Cell::new(None) };
    }
    LAST_LOG.with(|last| {
        let now = std::time::Instant::now();
        if last.get().is_some_and(|last| now.duration_since(last).as_secs() < 5) {
            return;
        }
        last.set(Some(now));
        let d = audio.diagnostics();
        let rate = u64::from(audio.output_sample_rate().max(1));
        tracing::debug!(
            scheduled = d.scheduled_sound_count,
            late_frames = d.scheduling_late_frames,
            max_late_frames = d.scheduling_max_late_frames,
            max_late_us = d.scheduling_max_late_frames.saturating_mul(1_000_000) / rate,
            "audio scheduling latency (cumulative callback arrival)"
        );
    });
}

#[cfg(test)]
fn queue_keysound_volumes(pending: &mut Vec<(SoundId, f32)>, volumes: &[(SoundId, f32)]) {
    for &(sound_id, volume) in volumes {
        if let Some((_, pending_volume)) =
            pending.iter_mut().find(|(pending_sound_id, _)| *pending_sound_id == sound_id)
        {
            *pending_volume = volume;
        } else {
            pending.push((sound_id, volume));
        }
    }
}

/// HCN 早離し時のミュート/復帰など、フレームで発生したキー音音量変更を
/// audio engine に反映する。audio callback との競合時は次フレームへ retry する。
#[cfg(test)]
fn flush_keysound_volumes_nonblocking(
    audio: &SharedAudioEngine,
    pending: &mut Vec<(SoundId, f32)>,
) -> Result<()> {
    if pending.is_empty() {
        return Ok(());
    }
    match audio.try_lock() {
        Ok(mut audio) => {
            for (sound_id, volume) in pending.drain(..) {
                audio.set_sound_volume(sound_id, volume);
            }
            Ok(())
        }
        Err(TryLockError::WouldBlock) => Ok(()),
        Err(TryLockError::Poisoned(_)) => Err(anyhow!("audio engine lock poisoned")),
    }
}

#[cfg(test)]
fn flush_keysound_volumes_commands(
    audio: &AudioEngineHandle,
    pending: &mut Vec<(SoundId, f32)>,
) -> Result<()> {
    if pending.is_empty() {
        return Ok(());
    }
    let mut remaining = Vec::new();
    for (sound_id, volume) in pending.drain(..) {
        if !audio.set_sound_volume(sound_id, volume) {
            remaining.push((sound_id, volume));
        }
    }
    *pending = remaining;
    Ok(())
}

pub fn advance_running_play_session(
    running: &mut RunningPlaySession,
) -> Result<FrameOutput<RenderSnapshot>> {
    log_audio_scheduling_latency(&running.audio.engine);
    let frame = running.gameplay.advance(&running.audio.engine);
    let mut output = frame_output_from_session_frame_cached(
        &running.session,
        frame,
        running.best_ex_score,
        running.best_ghost.as_deref(),
        running.target_ex_score,
        &running.bga_frames,
        &running.render_snapshot_cache,
    );
    apply_play_arrange_to_snapshot(&mut output.render_snapshot, &running.applied_arrange);
    apply_running_play_target_to_snapshot(&mut output.render_snapshot, running);
    apply_running_play_mode_to_snapshot(&mut output.render_snapshot, running);
    Ok(output)
}

pub fn advance_running_play_session_until_result(
    running: &mut RunningPlaySession,
    score_db: &mut ScoreDatabase,
    network_db: &mut NetworkDatabase,
    profile_paths: &ProfilePaths,
    replay_config: &ReplayConfig,
    ir_config: &IrConfig,
    played_at: i64,
) -> Result<PlayAdvanceOutcome> {
    log_audio_scheduling_latency(&running.audio.engine);
    let session_frame = running.gameplay.advance(&running.audio.engine);
    let mut frame = frame_output_from_session_frame_cached(
        &running.session,
        session_frame,
        running.best_ex_score,
        running.best_ghost.as_deref(),
        running.target_ex_score,
        &running.bga_frames,
        &running.render_snapshot_cache,
    );
    apply_play_arrange_to_snapshot(&mut frame.render_snapshot, &running.applied_arrange);
    apply_running_play_target_to_snapshot(&mut frame.render_snapshot, running);
    apply_running_play_mode_to_snapshot(&mut frame.render_snapshot, running);
    running.result_graph.record_frame(&frame);
    if matches!(frame.state, PlayState::Finished | PlayState::Failed) {
        let chart_length_ms = running.chart_length_ms;
        let play_duration_ms = running.finish_play_duration_ms();
        let mut finished = finish_session_result_once(
            &mut running.finished,
            score_db,
            network_db,
            FinishSessionResultOnceRequest {
                profile_paths,
                replay_config,
                ir_config,
                session: &running.gameplay.session,
                played_at,
                applied_arrange: &running.applied_arrange,
                source_ln_profile: running.source_ln_profile,
                chart_length_ms: Some(chart_length_ms),
                play_duration_ms: Some(play_duration_ms),
                target_ex_score: running.target_ex_score,
                target_name: &running.target,
                score_key: running.score_key,
                practice_mode: running.practice_mode,
                finish_mode: FinishResultMode::Normal,
            },
        )?;
        finished.summary.skin_attempt = running.skin_attempt;
        finished.summary.graph =
            std::sync::Arc::new(running.result_graph.snapshot_for_session(&running.session));
        running.finished = Some(finished.clone());
        // ここでは音声を止めない。スケジュール済みの BGM/キー音は
        // オーディオ出力スレッド側で曲の最後まで鳴り切る。出力の解放は
        // リザルト画面側 (advance_draining_audio) がドレイン完了後に行う。
        return Ok(PlayAdvanceOutcome::Finished { frame, finished: Box::new(finished) });
    }

    Ok(PlayAdvanceOutcome::Playing(frame))
}

fn apply_running_play_target_to_snapshot(
    snapshot: &mut RenderSnapshot,
    running: &RunningPlaySession,
) {
    snapshot.target = running.target.clone();
}

fn apply_running_play_mode_to_snapshot(
    snapshot: &mut RenderSnapshot,
    running: &RunningPlaySession,
) {
    snapshot.skin_attempt = running.skin_attempt;
    snapshot.rule_mode_index = crate::skin_extension::rule_mode_index(running.score_key.rule_mode);
    snapshot.ln_score_policy_index =
        Some(crate::skin_extension::ln_score_policy_index(running.score_key.ln_policy));
    snapshot.practice_mode = running.practice_mode;
    snapshot.score_save_enabled = !snapshot.autoplay
        && !snapshot.replay_playback
        && !running.practice_mode
        && !running.score_save_disabled
        && running.session.assist.score_update_enabled();
}

/// `play_ending` 中に skin 側へ渡す壁時計ベースの timer 値。
#[derive(Debug, Clone, Copy)]
pub struct PlayEndingSkinTimers {
    pub play_elapsed_time: TimeUs,
    pub ready_elapsed_time: Option<TimeUs>,
    pub backbmp_background: bool,
    pub failed_elapsed_ms: Option<i32>,
    pub music_end_elapsed_ms: Option<i32>,
    pub fadeout_elapsed_ms: Option<i32>,
}

/// 終了演出中に gameplay を止めたまま、オーディオクロックに追従して描画 snapshot を更新する。
pub fn refresh_play_ending_snapshot(
    running: &mut RunningPlaySession,
    timers: PlayEndingSkinTimers,
) -> RenderSnapshot {
    running.gameplay.flush_audio(&running.audio.engine);
    let mut snapshot = refresh_play_ending_snapshot_with_session_cached(
        &mut running.gameplay.session,
        running.best_ex_score,
        running.best_ghost.as_deref(),
        running.target_ex_score,
        &running.bga_frames,
        timers,
        &running.render_snapshot_cache,
    );
    apply_play_arrange_to_snapshot(&mut snapshot, &running.applied_arrange);
    apply_running_play_target_to_snapshot(&mut snapshot, running);
    apply_running_play_mode_to_snapshot(&mut snapshot, running);
    snapshot
}

pub(crate) fn apply_play_arrange_to_snapshot(
    snapshot: &mut RenderSnapshot,
    applied: &AppliedArrange,
) {
    snapshot.arrange = applied.arrange.as_str().to_string();
    snapshot.arrange_2p = applied.arrange_2p.as_str().to_string();
    snapshot.lane_shuffle_pattern = applied.pattern.clone().unwrap_or_default();
}

pub fn refresh_play_ending_snapshot_with_session(
    session: &mut GameSession,
    best_ex_score: Option<u32>,
    best_ghost: Option<&[u8]>,
    target_ex_score: Option<u32>,
    bga_frames: &BgaFrameCatalog,
    timers: PlayEndingSkinTimers,
) -> RenderSnapshot {
    let cache = PlayRenderSnapshotCache::from_chart(&session.chart);
    refresh_play_ending_snapshot_with_session_cached(
        session,
        best_ex_score,
        best_ghost,
        target_ex_score,
        bga_frames,
        timers,
        &cache,
    )
}

pub fn refresh_play_ending_snapshot_with_session_cached(
    session: &mut GameSession,
    best_ex_score: Option<u32>,
    best_ghost: Option<&[u8]>,
    target_ex_score: Option<u32>,
    bga_frames: &BgaFrameCatalog,
    timers: PlayEndingSkinTimers,
    cache: &PlayRenderSnapshotCache,
) -> RenderSnapshot {
    let times = compute_frame_times(session);
    apply_auto_key_release(session, times.audio_now);
    update_recent_judgements(session, &[], times.audio_now);
    update_recent_inputs(session, &[], times.audio_now);

    let mut snapshot = build_render_snapshot_with_target_and_bga_frames_cached(
        session,
        times.audio_now,
        &session.recent_judgements,
        best_ex_score,
        best_ghost,
        target_ex_score,
        bga_frames,
        cache,
    );
    snapshot.play_elapsed_time = timers.play_elapsed_time;
    snapshot.ready_elapsed_time = timers.ready_elapsed_time;
    snapshot.backbmp_background = timers.backbmp_background;
    snapshot.failed_elapsed_ms = timers.failed_elapsed_ms;
    snapshot.music_end_elapsed_ms = timers.music_end_elapsed_ms;
    snapshot.fadeout_elapsed_ms = timers.fadeout_elapsed_ms;
    crate::screens::play_snapshot::refresh_play_skin_visuals(&mut snapshot, session);
    snapshot
}

#[cfg(test)]
#[path = "play_loop/tests.rs"]
mod tests;
