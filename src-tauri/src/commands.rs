use std::time::Duration;
use tokio::time::sleep;

use tauri::{api::notification::Notification, AppHandle};

#[tauri::command]
pub async fn start_timer(app_handle: AppHandle, timer_seconds: u64) {
    println!("starting the timer");

    sleep(Duration::from_secs(timer_seconds)).await;

    println!("Sending Notification (notifications are not visible during dev mode)");
    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();
}
