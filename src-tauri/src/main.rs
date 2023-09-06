#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::*;
use app::handlers::*;
use app::persistent::*;

use tauri::RunEvent;
use tauri::{SystemTray, SystemTrayMenu, CustomMenuItem};

fn main() {
    let quit = CustomMenuItem::new("quit", "Quit");
    let open = CustomMenuItem::new("open", "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(quit);
    
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    let state = AppState::new(load_projects());

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![remove_project])
        .manage(state)
        .setup(|app| {
            update_loop(app.handle());
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => { }
        });
}