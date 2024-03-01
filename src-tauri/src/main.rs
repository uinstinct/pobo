// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::path::app_data_dir;

use crate::{
    helpers::check_timestamp_and_get_session_counter,
    store::{PoboStore, SessionStore},
};

mod commands;
mod helpers;
mod state;
mod store;
mod tray_menu;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(state::TimerState::new())
        .manage(state::StopwatchState::new())
        .system_tray(tray_menu::get_tray_menu())
        .on_system_tray_event(tray_menu::handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            commands::start_timer,
            commands::resync_timer,
            commands::stop_timer,
            commands::resync_stopwatch,
            commands::stop_stopwatch,
            commands::resync_session_counter
        ])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            println!(
                "the local dir was data_dir -> {:#?} ",
                app_data_dir(&app.config())
            );
            SessionStore::load(app.handle());
            SessionStore::set_session_counter(
                &app.handle(),
                check_timestamp_and_get_session_counter(&app.handle()),
            );
            SessionStore::set_timestamp(&app.handle());

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
