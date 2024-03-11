use chrono::offset;
use tauri::{AppHandle, Manager};

use crate::store::SessionStore;

pub fn check_timestamp_and_get_session_counter(app_handle: &AppHandle) -> u64 {
    let last_session_timestamp =
        SessionStore::get_timestamp(&app_handle).unwrap_or(offset::Local::now());
    let current_session_timestamp = offset::Local::now();
    if (current_session_timestamp - last_session_timestamp).num_hours() > 2 {
        return 0;
    }
    SessionStore::get_session_counter(&app_handle).unwrap_or(0)
}

pub fn bring_window_to_focus(app_handle: &AppHandle) {
    let windows = app_handle.windows();
    if windows.len() > 0 {
        let current_window = windows.values().next().unwrap();
        current_window.set_focus().unwrap();
    } else {
        tauri::WindowBuilder::from_config(
            &app_handle.clone(),
            app_handle.config().tauri.windows.get(0).unwrap().clone(),
        )
        .center()
        .build()
        .unwrap();
    }
}
