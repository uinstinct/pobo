use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

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
                let windows = app_handle.windows();
                if windows.len() > 0 {
                    app_handle.show().unwrap();
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
            "quit" => std::process::exit(0),
            _ => {}
        },
        _ => {}
    };
}
