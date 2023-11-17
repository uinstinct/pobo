use tauri::async_runtime::JoinHandle;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};

/// Session Timer
pub struct TimerState {
    pub start_instant: Mutex<Option<Instant>>,
    pub timer_seconds: Mutex<Option<u64>>,
    pub notify_timer_finish_task: Mutex<Option<JoinHandle<()>>>,
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
}

/// Cooldown Stopwatch
pub struct StopwatchState {
    pub start_instant: Mutex<Option<Instant>>,
    pub notify_stopwatch_task: Mutex<Option<JoinHandle<()>>>,
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

        let start_instant = Instant::now();

        let stopwatch_state = app_handle.state::<Self>();

        let mut start_intant_state = stopwatch_state.start_instant.lock().await;
        *start_intant_state = Some(start_instant);

        let notify_stopwatch_task = tauri::async_runtime::spawn(Self::tick_and_notify_stop(
            app_handle.clone(),
            start_instant,
        ));

        let mut notify_stopwatch_task_state = stopwatch_state.notify_stopwatch_task.lock().await;
        *notify_stopwatch_task_state = Some(notify_stopwatch_task);
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
