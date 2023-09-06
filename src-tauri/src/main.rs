#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::*;
use app::handlers::*;

use tauri::Manager;

fn main() {
    let state = AppState::new(load());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![remove_project])
        .manage(state)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            update_loop(main_window, app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}