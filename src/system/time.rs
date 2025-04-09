use std::time::{Duration, Instant};

struct TimeManager {
    instant: Instant,
}

impl TimeManager {
    pub fn new() -> Self {
        TimeManager {
            instant: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.instant.elapsed()
    }
}
