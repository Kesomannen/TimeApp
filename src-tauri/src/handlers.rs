use crate::{AppState, send_update};

#[tauri::command]
pub fn remove_project(name: String, state: tauri::State<'_, AppState>, window: tauri::Window) {
    let mut projects = state.projects.lock().unwrap();
    projects.remove(&name);

    if let Err(err) = send_update(&projects, &window) {
        eprintln!("Error sending update: {}", err);
    }
}