use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn get_tray_menu() -> SystemTray {
    let tray_menu =
        SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit").accelerator("Cmd + Q"));
    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_system_tray_event(_app_handle: &AppHandle, tray_event: SystemTrayEvent) {
    match tray_event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => std::process::exit(0),
            _ => {}
        },
        _ => {}
    };
}