use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct TimerState {
    pub start_instant: Mutex<Option<Instant>>,
    pub timer_seconds: Mutex<Option<u64>>,
}

impl TimerState {
    pub fn new() -> Self {
        TimerState {
            start_instant: Mutex::new(None),
            timer_seconds: Mutex::new(None),
        }
    }
}
