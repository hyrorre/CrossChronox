//! The renderer-independent owner of one chart's mutable gameplay state.
//! Audio callbacks consume commands only; all judgement stays on this owner.
use bmz_audio::command::AudioEngineHandle;
use bmz_audio::queue::{AudioScheduler, ScheduledSoundQueue};
use bmz_core::ids::SoundId;

use crate::session::{GameSession, SessionFrame, advance_session_frame};

pub struct GameplayRuntime {
    pub session: GameSession,
    pub pending_audio: ScheduledSoundQueue,
    pub pending_keysound_volumes: Vec<(SoundId, f32)>,
}

impl GameplayRuntime {
    pub fn new(session: GameSession) -> Self {
        Self {
            session,
            pending_audio: ScheduledSoundQueue::new(),
            pending_keysound_volumes: Vec::new(),
        }
    }

    pub fn advance(&mut self, audio: &AudioEngineHandle) -> SessionFrame {
        let frame = advance_session_frame(&mut self.session, &mut self.pending_audio);
        for &(id, volume) in &frame.keysound_volumes {
            if let Some((_, pending)) =
                self.pending_keysound_volumes.iter_mut().find(|(pending_id, _)| *pending_id == id)
            {
                *pending = volume;
            } else {
                self.pending_keysound_volumes.push((id, volume));
            }
        }
        self.flush_audio(audio);
        frame
    }

    pub fn flush_audio(&mut self, audio: &AudioEngineHandle) {
        if !self.pending_audio.is_empty() {
            let sounds = self.pending_audio.drain_all().collect();
            match audio.try_schedule_all(sounds) {
                Ok(()) => crate::session::latency::audio_enqueued(),
                Err(sounds) => {
                    for sound in sounds {
                        self.pending_audio.schedule(sound);
                    }
                }
            }
        }
        self.pending_keysound_volumes.retain(|&(id, volume)| !audio.set_sound_volume(id, volume));
    }
}
