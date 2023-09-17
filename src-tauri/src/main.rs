#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use unity_dev_timer::*;
use unity_dev_timer::config::Options;
use unity_dev_timer::handlers::*;
use unity_dev_timer::persistent::*;

use tauri::RunEvent;
use tauri::{SystemTray, SystemTrayMenu, CustomMenuItem};

fn main() {
    let hide = CustomMenuItem::new("hide", "Hide");
    let quit = CustomMenuItem::new("quit", "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(quit);
    
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    let state = AppState::new(
        Options::new(load_config()),
        load_projects()
    );

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![remove_project, get_key, set_key])
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