use rustzx_core::host::Stopwatch;
// use std::time::{Duration, Instant};
use core::time::Duration;

pub struct InstantStopwatch {
    // timestamp: Instant,
}

impl Default for InstantStopwatch {
    fn default() -> Self {
        Self {
            // timestamp: Instant::now(),
        }
    }
}

impl Stopwatch for InstantStopwatch {
    fn new() -> Self {
        Self::default()
    }

    fn measure(&self) -> Duration {
        // self.timestamp.elapsed()
        Duration::from_millis(100)
    }
}
