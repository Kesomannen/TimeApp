use tauri::AppHandle;

use crate::{AppState, send_update};

#[tauri::command]
pub fn remove_project(name: String, state: tauri::State<'_, AppState>, app: AppHandle) {
    let mut projects = state.projects.lock().unwrap();
    projects.remove(&name);

    if let Err(err) = send_update(&projects, &app) {
        eprintln!("Error sending update: {}", err);
    }
}