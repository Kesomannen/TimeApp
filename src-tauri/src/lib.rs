use std::{thread, time::Duration, sync::Mutex, collections::HashMap};

use tauri::{Manager, AppHandle, SystemTrayEvent};
use sysinfo::{ProcessExt, System, SystemExt, PidExt};
use winapi::um::winuser::*;

use persistent::save_projects;

pub mod handlers;
pub mod persistent;

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Project {
    name: String,
    time: Duration,
    open: bool,
}

#[derive(Default)]
pub struct AppState {
    projects: Mutex<HashMap<String, Project>>,
}

impl AppState {
    pub fn new(projects: HashMap<String, Project>) -> Self {
        Self {
            projects: Mutex::new(projects),
        }
    }
}

#[derive(serde::Serialize, Clone, Default)]
struct UpdatePayload {
    projects: Vec<Project>,
}

pub fn update_loop(app: tauri::AppHandle) {
    let mut system = System::new_all();
    thread::spawn(move || loop {
        thread::sleep(UPDATE_INTERVAL);

        system.refresh_all();

        let state = app.state::<AppState>();
        let mut projects = state.projects.lock().unwrap();

        for project in projects.values_mut() {
            project.open = false;
        }

        for process in system.processes_by_exact_name("Unity.exe") {
            let open_project_names = get_process_windows(process.pid().as_u32()).into_iter()
                .filter(|title| title.contains("- Unity"))
                .map(|title| title.split("-").next().unwrap().trim().to_string());

            for project_name in open_project_names {
                let project = projects
                    .entry(project_name.clone())
                    .or_insert_with(|| Project { 
                        name: project_name, 
                        time: Duration::from_secs(0),
                        open: true,
                    });

                project.time += UPDATE_INTERVAL;
                project.open = true;
            }
        }

        if let Err(err) = send_update(&projects, &app) {
            println!("Failed to send update: {}", err);
        }
    });
}

fn send_update(projects: &HashMap<String, Project>, app: &AppHandle) -> Result<(), tauri::Error> {
    let payload = UpdatePayload {
        projects: projects.values().cloned().collect(),
    };

    app.emit_all("update", payload)?;
    save_projects(projects);

    Ok(())
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { .. } => todo!(),
        SystemTrayEvent::LeftClick { .. } => todo!(),
        SystemTrayEvent::RightClick { .. } => todo!(),
        SystemTrayEvent::DoubleClick { .. } => todo!(),
        _ => todo!(),
    }
}

fn get_process_windows(process_id: u32) -> Vec<String> {
    let mut windows = Vec::new();

    unsafe {
        let mut window = GetTopWindow(std::ptr::null_mut());
        while window != std::ptr::null_mut() {
            let mut pid = 0;
            GetWindowThreadProcessId(window, &mut pid);
            if pid == process_id {
                let mut title = [0u16; 1024];
                GetWindowTextW(window, title.as_mut_ptr(), 1024);
                windows.push(String::from_utf16_lossy(&title));
            }

            window = GetWindow(window, GW_HWNDNEXT);
        }
    }

    windows
}