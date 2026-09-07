use std::collections::VecDeque;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

use bmz_gameplay::input::backend::{DeviceInputEvent, InputBackend, InputEventSink};

#[derive(Debug, Clone, Default)]
pub struct SharedInputBackend {
    buffer: Arc<Mutex<VecDeque<DeviceInputEvent>>>,
    waker: Arc<Mutex<Option<std::thread::Thread>>>,
    overflow: Arc<AtomicU64>,
}

impl SharedInputBackend {
    pub const CAPACITY: usize = 16_384;
    pub fn same_source(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.buffer, &other.buffer)
    }
    pub fn overflow_count(&self) -> u64 {
        self.overflow.load(Ordering::Relaxed)
    }
    pub fn push_shared_event(&self, event: DeviceInputEvent) {
        if let Ok(mut buffer) = self.buffer.lock() {
            if buffer.len() == Self::CAPACITY {
                if self.overflow.fetch_add(1, Ordering::Relaxed) == 0 {
                    tracing::error!(
                        capacity = Self::CAPACITY,
                        "timestamped gameplay input queue overflow"
                    );
                }
            } else {
                buffer.push_back(event);
            }
        }
        if let Ok(waker) = self.waker.lock()
            && let Some(waker) = &*waker
        {
            waker.unpark();
        }
    }
}

impl InputBackend for SharedInputBackend {
    fn set_waker(&mut self, waker: Option<std::thread::Thread>) {
        if let Ok(mut target) = self.waker.lock() {
            *target = waker;
        }
    }
    fn drain_events(&mut self) -> Vec<DeviceInputEvent> {
        self.buffer.lock().map(|mut buffer| buffer.drain(..).collect()).unwrap_or_default()
    }
}

impl InputEventSink for SharedInputBackend {
    fn push_event(&mut self, event: DeviceInputEvent) {
        self.push_shared_event(event);
    }
}

#[cfg(test)]
mod tests {
    use bmz_core::input::InputKind;
    use bmz_gameplay::input::backend::{DeviceId, DeviceTimestamp, PhysicalControl};

    use super::*;

    #[test]
    fn cloned_shared_input_backend_drains_events_once() {
        let event_source = SharedInputBackend::default();
        let mut game_backend = event_source.clone();

        event_source.push_shared_event(DeviceInputEvent {
            device: DeviceId(0),
            control: PhysicalControl::KeyboardKey("Z".to_string()),
            kind: InputKind::Press,
            timestamp: DeviceTimestamp::Unknown,
            bounce_policy: Default::default(),
        });

        let events = game_backend.drain_events();
        assert_eq!(events.len(), 1);
        assert!(game_backend.drain_events().is_empty());
    }
}
