use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

use crate::state::TimerState;
use tauri::State;

pub fn get_tray_menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("quit", "Quit").accelerator("Cmd + Q"));
    SystemTray::new().with_menu(tray_menu)
}

fn resync_timer(app_handle: &AppHandle) {
    println!("resyncing timer");

    app_handle
        .emit_all("resync_timer", "something else here")
        .unwrap();

    let timer_state: State<TimerState> = app_handle.state();

    let start_instant = tauri::async_runtime::block_on(timer_state.start_instant.lock());

    if let Some(start_instant) = *start_instant {
        app_handle
            .emit_all("resync_timer", start_instant.elapsed())
            .unwrap();
    } else {
        app_handle
            .emit_all(
                "resync_timer",
                "writing something here, need to manage state",
            )
            .unwrap()
    }
}

pub fn handle_system_tray_event(app_handle: &AppHandle, tray_event: SystemTrayEvent) {
    match tray_event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
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

                    resync_timer(app_handle);
                }
            }
            "quit" => std::process::exit(0),
            _ => {}
        },
        _ => {}
    };
}
