use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

use crate::helpers::bring_window_to_focus;

pub fn get_tray_menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("quit", "Quit").accelerator("Cmd + Q"));
    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_system_tray_event(app_handle: &AppHandle, tray_event: SystemTrayEvent) {
    match tray_event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                bring_window_to_focus(&app_handle);
            }
            "quit" => std::process::exit(0),
            _ => {}
        },
        _ => {}
    };
}
