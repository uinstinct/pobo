use tokio::time::{interval as tokio_interval, Duration, Instant};

use tauri::{api::notification::Notification, AppHandle};

use crate::state::TimerState;

/// store the starting instant in state, start the interval and send the notification when the duration is elapsed
///
/// drop the mutexes because it will not be released until the interval is over
#[tauri::command]
pub async fn start_timer(
    timer_seconds: u64,
    timer_state: tauri::State<'_, TimerState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    println!("starting the timer");

    let start_instant = Instant::now();

    let mut start_instant_state = timer_state.start_instant.lock().await;
    *start_instant_state = Some(start_instant);
    drop(start_instant_state);

    let mut timer_seconds_state = timer_state.timer_seconds.lock().await;
    *timer_seconds_state = Some(timer_seconds);
    drop(timer_seconds_state);

    let timer_duration = Duration::from_secs(timer_seconds);

    let mut interval = tokio_interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        if start_instant.elapsed() > timer_duration {
            break;
        }
    }

    println!("Sending Notification (notifications are not visible during dev mode)");
    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();

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

    let start_instant = timer_state.start_instant.lock().await;
    let timer_seconds = timer_state.timer_seconds.lock().await;

    if let Some(start_instant) = *start_instant {
        return Ok(CurrentTimerState {
            elapsed: Some(start_instant.elapsed().as_secs()),
            timer_seconds: *timer_seconds,
        });
    }

    return Ok(CurrentTimerState {
        elapsed: None,
        timer_seconds: None,
    });
}
