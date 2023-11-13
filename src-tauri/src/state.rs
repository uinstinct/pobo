use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct TimerState {
    pub start_instant: Mutex<Option<Instant>>,
}

pub fn get_managed_timer_state() -> TimerState {
    TimerState {
        start_instant: Mutex::new(None),
    }
}
