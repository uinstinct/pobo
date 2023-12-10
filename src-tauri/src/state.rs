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
    run_task: Mutex<Option<JoinHandle<()>>>,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            start_instant: Mutex::new(None),
            timer_seconds: Mutex::new(None),
            /// used to cancel the timer when a new timer starts or the old timer is cancelled
            run_task: Mutex::new(None),
        }
    }

    pub async fn abort_run_task(&self) {
        let run_task = self.run_task.lock().await;

        if (*run_task).is_some() {
            (*run_task).as_ref().unwrap().abort();
        }
    }

    pub async fn reset(&self) {
        future::join3(
            self.set_start_instant(None),
            self.set_timer_seconds(None),
            self.abort_run_task(),
        )
        .await;
    }
}

/// Cooldown Stopwatch
#[derive(MutexGetSet)]
pub struct StopwatchState {
    start_instant: Mutex<Option<Instant>>,
    run_task: Mutex<Option<JoinHandle<()>>>,
}

impl StopwatchState {
    pub fn new() -> Self {
        Self {
            start_instant: Mutex::new(None),
            run_task: Mutex::new(None),
        }
    }

    pub async fn abort_notify_stopwatch_finish_task(&self) {
        let notify_stopwatch_finish_task = self.run_task.lock().await;

        if (*notify_stopwatch_finish_task).is_some() {
            (*notify_stopwatch_finish_task).as_ref().unwrap().abort();
        }
    }

    pub async fn reset(&self) {
        future::join(
            self.set_start_instant(None),
            self.abort_notify_stopwatch_finish_task(),
        )
        .await;
    }
}
