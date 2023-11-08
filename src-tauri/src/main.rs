// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use tauri::api::notification::Notification;
use tokio::time::sleep;

mod tray_menu;

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
        .system_tray(tray_menu::get_tray_menu())
        .on_system_tray_event(tray_menu::handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![start_timer])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                println!("exiting tauri application");
                api.prevent_exit();
            }
            _ => {}
        });
}
