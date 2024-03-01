use chrono::offset;
use tauri::AppHandle;

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
