// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod tray_menu;

fn main() {
    tauri::Builder::default()
        .manage(state::TimerState::new())
        .system_tray(tray_menu::get_tray_menu())
        .on_system_tray_event(tray_menu::handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            commands::start_timer,
            commands::resync_timer
        ])
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
