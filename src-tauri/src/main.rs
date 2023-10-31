// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use tauri::api::notification::Notification;
use tokio::time::sleep;

#[tauri::command]
async fn start_timer(app_handle: tauri::AppHandle, timer_seconds: u64) {
    println!("starting the timer");

    sleep(Duration::from_secs(timer_seconds)).await;

    let _ = Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Timer Complete")
        .body("The timer has completed")
        .show();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_timer])
        // .system_tray(tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
