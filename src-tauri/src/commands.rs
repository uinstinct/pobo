use std::time::Duration;
use tokio::time::Instant;

use tauri::{api::notification::Notification, AppHandle};

#[tauri::command]
pub async fn start_timer(app_handle: AppHandle, timer_seconds: u64) -> bool {
    println!("starting the timer");

    let start_instant = Instant::now();

    let timer_duration = Duration::from_secs(timer_seconds);

    loop {
        if start_instant.elapsed() > timer_duration {
            break;
        }
    }

    println!("Sending Notification (notifications are not visible during dev mode)");
    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();

    true
}