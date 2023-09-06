use tauri::AppHandle;

use crate::{AppState, send_update};

#[tauri::command]
pub fn remove_project(name: String, state: tauri::State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    let mut projects = state.projects.lock().unwrap();
    projects.remove(&name);
    send_update(&projects, &app).map_err(|err| err.to_string())?;
    
    Ok(())
}