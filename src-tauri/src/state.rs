use derive_macro::MutexGetSet;
use tauri::async_runtime::JoinHandle;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};

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

    pub async fn start(app_handle: &AppHandle) {
        println!("stopwatch started");
        app_handle.emit_all("stopwatch_started", ()).unwrap();
        let stopwatch_state = app_handle.state::<Self>();

        let start_instant = Instant::now();
        stopwatch_state.set_start_instant(Some(start_instant)).await;

        let notify_stopwatch_task = tauri::async_runtime::spawn(Self::tick_and_notify_stop(
            app_handle.clone(),
            start_instant,
        ));

        stopwatch_state
            .set_notify_stopwatch_task(Some(notify_stopwatch_task))
            .await;
    }

    async fn tick_and_notify_stop(app_handle: AppHandle, start_instant: Instant) {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            if start_instant.elapsed() > Duration::from_secs(1 * 10) {
                break;
            }
        }

        app_handle.emit_all("stopwatch_finished", ()).unwrap();
        println!("stopwatch cooldown duration finished")
    }
}
