use futures::future;
use tokio::time::{interval as tokio_interval, Duration, Instant};

use tauri::{api::notification::Notification, AppHandle, Manager};

use crate::state::{StopwatchState, TimerState};

async fn start_timer_task(app_handle: AppHandle, start_instant: Instant, timer_duration: Duration) {
    let mut interval = tokio_interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        if start_instant.elapsed() > timer_duration {
            break;
        }
    }

    let timer_state = app_handle.state::<TimerState>();

    future::join(
        timer_state.set_start_instant(None),
        timer_state.set_timer_seconds(None),
    )
    .await;

    println!("Sending Notification (notifications are not visible during dev mode)");
    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();

    start_stopwatch(app_handle.state::<StopwatchState>(), app_handle.clone())
        .await
        .unwrap();
}

#[tauri::command]
pub async fn start_timer(
    timer_seconds: u64,
    timer_state: tauri::State<'_, TimerState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    println!("starting the timer");

    let start_instant = Instant::now();

    future::join3(
        timer_state.set_start_instant(Some(start_instant)),
        timer_state.set_timer_seconds(Some(timer_seconds)),
        timer_state.abort_run_task(),
    )
    .await;

    let timer_run_task = tauri::async_runtime::spawn(start_timer_task(
        app_handle,
        start_instant,
        Duration::from_secs(timer_seconds),
    ));
    timer_state.set_run_task(Some(timer_run_task)).await;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct CurrentTimerState {
    pub elapsed: Option<u64>,
    pub timer_seconds: Option<u64>,
}

#[tauri::command]
pub async fn resync_timer(
    timer_state: tauri::State<'_, TimerState>,
) -> Result<CurrentTimerState, ()> {
    println!("resyncing the timer");

    let (start_instant_state, timer_seconds_state) = future::join(
        timer_state.get_start_instant(),
        timer_state.get_timer_seconds(),
    )
    .await;

    let current_timer_state = if let (Some(start_instant), Some(timer_seconds)) =
        (start_instant_state, timer_seconds_state)
    {
        Ok(CurrentTimerState {
            elapsed: Some(start_instant.elapsed().as_secs()),
            timer_seconds: Some(timer_seconds),
        })
    } else {
        Ok(CurrentTimerState {
            elapsed: None,
            timer_seconds: None,
        })
    };

    println!("current_timer_state was {:#?}", current_timer_state);

    current_timer_state
}

#[tauri::command]
pub async fn stop_timer(
    timer_state: tauri::State<'_, TimerState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    println!("manually stoping timer and starting stopwatch");
    timer_state.reset().await;
    start_stopwatch(app_handle.state::<StopwatchState>(), app_handle.clone())
        .await
        .unwrap();
    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct CurrentStopWatchState {
    pub elapsed: Option<u64>,
}

async fn notify_stopwatch_finished(app_handle: &AppHandle) {
    app_handle.emit_all("stopwatch_finished", ()).unwrap();
}

async fn start_stopwatch_task(app_handle: AppHandle, start_instant: Instant) {
    let mut interval = tokio_interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        if start_instant.elapsed() > Duration::from_secs(1 * 10) {
            break;
        }
    }

    let stopwatch_state = app_handle.state::<StopwatchState>();
    stopwatch_state.set_start_instant(None).await;

    notify_stopwatch_finished(&app_handle).await;
    println!("stopwatch cooldown duration finished")
}

#[tauri::command]
async fn start_stopwatch(
    stopwatch_state: tauri::State<'_, StopwatchState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    println!("stopwatch started");
    app_handle.emit_all("stopwatch_started", ()).unwrap();

    let start_instant = Instant::now();
    stopwatch_state.set_start_instant(Some(start_instant)).await;

    let stopwatch_run_task =
        tauri::async_runtime::spawn(start_stopwatch_task(app_handle, start_instant));

    stopwatch_state.set_run_task(Some(stopwatch_run_task)).await;

    Ok(())
}

#[tauri::command]
pub async fn resync_stopwatch(
    stopwatch_state: tauri::State<'_, StopwatchState>,
) -> Result<CurrentStopWatchState, ()> {
    println!("resyncing the stopwatch");

    let start_instant_state = stopwatch_state.get_start_instant().await;

    let elapsed = if let Some(start_instant) = start_instant_state {
        Some(start_instant.elapsed().as_secs())
    } else {
        None
    };

    Ok(CurrentStopWatchState { elapsed })
}

#[tauri::command]
pub async fn stop_stopwatch(
    stopwatch_state: tauri::State<'_, StopwatchState>,
    app_handle: AppHandle,
) -> Result<(), ()> {
    println!("manually stopping the stopwatch");

    stopwatch_state.reset().await;
    notify_stopwatch_finished(&app_handle).await;

    Ok(())
}
