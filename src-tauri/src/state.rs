use derive_macro::MutexGetSet;
use futures::future;
use tauri::async_runtime::JoinHandle;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// Session Timer
#[derive(MutexGetSet)]
pub struct TimerState {
    start_instant: Mutex<Option<Instant>>,
    timer_seconds: Mutex<Option<u64>>,
    notify_timer_finish_task: Mutex<Option<JoinHandle<()>>>,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            start_instant: Mutex::new(None),
            timer_seconds: Mutex::new(None),
            /// used to cancel the timer when a new timer starts or the old timer is cancelled
            notify_timer_finish_task: Mutex::new(None),
        }
    }

    pub async fn abort_notify_timer_finish_task(&self) {
        let notify_timer_finish_task = self.notify_timer_finish_task.lock().await;

        if (*notify_timer_finish_task).is_some() {
            (*notify_timer_finish_task).as_ref().unwrap().abort();
        }
    }

    pub async fn reset(&self) {
        future::join3(
            self.set_start_instant(None),
            self.set_timer_seconds(None),
            self.abort_notify_timer_finish_task(),
        )
        .await;
    }
}

/// Cooldown Stopwatch
#[derive(MutexGetSet)]
pub struct StopwatchState {
    start_instant: Mutex<Option<Instant>>,
    notify_stopwatch_task: Mutex<Option<JoinHandle<()>>>,
}

impl StopwatchState {
    pub fn new() -> Self {
        Self {
            start_instant: Mutex::new(None),
            notify_stopwatch_task: Mutex::new(None),
        }
    }
}
