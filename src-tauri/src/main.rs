#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::*;
use app::handlers::*;
use app::persistent::*;

use tauri::{SystemTray, SystemTrayMenu, CustomMenuItem};

fn main() {
    let quit = CustomMenuItem::new("quit", "Quit");
    let open = CustomMenuItem::new("open", "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(quit);
    
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    let state = AppState::new(load_projects());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![remove_project])
        .manage(state)
        .setup(|app| {
            update_loop(app.handle());
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(handle_system_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}