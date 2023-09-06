#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::*;
use app::handlers::*;
use app::persistent::*;

use tauri::{Manager, SystemTray, SystemTrayMenu, CustomMenuItem};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);
    
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    let state = AppState::new(load_projects());

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![remove_project])
        .manage(state)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            update_loop(app.handle(), main_window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}