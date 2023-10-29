// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use tokio::time::{self, Instant};

const TIME_LEFT_EVENT: &str = "time_left";

#[tauri::command]
async fn start_timer(window: tauri::Window) {
    println!("starting the timer");
    let mut interval = time::interval(Duration::from_secs(1));
    let earlier = Instant::now();
    loop {
        let instant = interval.tick().await;
        window
            .emit(
                &TIME_LEFT_EVENT,
                format!("{} have elapsed", instant.duration_since(earlier).as_secs()),
            )
            .unwrap();
    }
}

fn main() {
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    // let tray_menu = SystemTrayMenu::new()
    //     .add_item(quit)
    //     .add_native_item(SystemTrayMenuItem::Separator)
    //     .add_item(hide);

    // let tray = SystemTray::new().with_menu(tray_menu);

    // tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_timer])
        // .system_tray(tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
