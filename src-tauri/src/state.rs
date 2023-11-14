use tauri::async_runtime::JoinHandle;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct TimerState {
    pub start_instant: Mutex<Option<Instant>>,
    pub timer_seconds: Mutex<Option<u64>>,
    pub notify_timer_finish_task: Mutex<Option<JoinHandle<()>>>,
}

impl TimerState {
    pub fn new() -> Self {
        TimerState {
            start_instant: Mutex::new(None),
            timer_seconds: Mutex::new(None),
            /// used to cancel the timer when a new timer starts or the old timer is cancelled
            notify_timer_finish_task: Mutex::new(None),
        }
    }
}
