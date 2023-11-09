use tokio::time::{interval as tokio_interval, Duration, Instant};

use tauri::{api::notification::Notification, AppHandle, Manager};

#[tauri::command]
pub async fn start_timer(app_handle: AppHandle, timer_seconds: u64) -> bool {
    println!("starting the timer");

    let start_instant = Instant::now();

    let timer_duration = Duration::from_secs(timer_seconds);

    let mut interval = tokio_interval(Duration::from_secs(1));
    let mut last_resync_timer_instant = start_instant.clone();

    loop {
        interval.tick().await;

        if start_instant.elapsed() > timer_duration {
            break;
        }

        if last_resync_timer_instant.elapsed() >= Duration::from_secs(60) {
            app_handle
                .emit_all("resync_timer", start_instant.elapsed().as_secs())
                .unwrap();
            last_resync_timer_instant = Instant::now();
        }
    }

    println!("Sending Notification (notifications are not visible during dev mode)");
    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();

    true
}
