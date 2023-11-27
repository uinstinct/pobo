use tokio::time::{interval as tokio_interval, Duration, Instant};

use tauri::{api::notification::Notification, AppHandle, Manager};

use crate::state::{StopwatchState, TimerState};

async fn notify_timer_finish(
    app_handle: AppHandle,
    start_instant: Instant,
    timer_duration: Duration,
) {
    let mut interval = tokio_interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        if start_instant.elapsed() > timer_duration {
            break;
        }
    }

    app_handle.emit_all("timer_finished", ()).unwrap();

    println!("Sending Notification (notifications are not visible during dev mode)");
    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();

    StopwatchState::start(&app_handle).await;
}

#[tauri::command]
pub async fn start_timer(
    timer_seconds: u64,
    timer_state: tauri::State<'_, TimerState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    println!("starting the timer");

    let start_instant = Instant::now();
    timer_state.set_start_instant(Some(start_instant)).await;

    timer_state.set_timer_seconds(Some(timer_seconds)).await;

    timer_state.abort_notify_timer_finish_task().await;

    let notify_timer_finish_task = tauri::async_runtime::spawn(notify_timer_finish(
        app_handle,
        start_instant,
        Duration::from_secs(timer_seconds),
    ));
    timer_state
        .set_notify_timer_finish_task(Some(notify_timer_finish_task))
        .await;

    Ok(())
}

#[derive(serde::Serialize)]
pub struct CurrentTimerState {
    pub elapsed: Option<u64>,
    pub timer_seconds: Option<u64>,
}

#[tauri::command]
pub async fn resync_timer(
    timer_state: tauri::State<'_, TimerState>,
) -> Result<CurrentTimerState, ()> {
    println!("resyncing the timer");

    let start_instant_state = timer_state.get_start_instant().await;
    let timer_seconds_state = timer_state.get_timer_seconds().await;

    if let (Some(start_instant), Some(timer_seconds)) = (start_instant_state, timer_seconds_state) {
        if start_instant.elapsed().as_secs() < timer_seconds {
            return Ok(CurrentTimerState {
                elapsed: Some(start_instant.elapsed().as_secs()),
                timer_seconds: Some(timer_seconds),
            });
        } else {
            timer_state.set_start_instant(None).await;
            timer_state.set_timer_seconds(None).await;
        }
    }

    return Ok(CurrentTimerState {
        elapsed: None,
        timer_seconds: None,
    });
}
