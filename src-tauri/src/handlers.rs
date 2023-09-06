use crate::{AppState, send_update};

#[tauri::command]
pub fn remove_project(name: String, state: tauri::State<'_, AppState>, window: tauri::Window) -> Result<(), String> {
    let mut projects = state.projects.lock().unwrap();
    projects.remove(&name);
    send_update(&projects, &window);
    
    Ok(())
}