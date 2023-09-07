use std::io;

use tauri::AppHandle;

use crate::{*, persistent::*};

#[tauri::command]
pub fn remove_project(name: String, state: tauri::State<'_, AppState>, app: AppHandle) {
    let mut projects = state.projects.lock().unwrap();
    projects.remove(&name);

    if let Err(err) = send_update(&projects, &app) {
        eprintln!("Error sending update: {}", err);
    }
}

#[tauri::command]
pub fn set_auto_start(enabled: bool) -> io::Result<()> {
    set_auto_startup(enabled)
}

#[tauri::command]
pub fn get_auto_start() -> io::Result<bool> {
    get_auto_startup()
}