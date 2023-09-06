use std::{thread, time::Duration, sync::Mutex, collections::HashMap, fs, path::PathBuf};

use directories::ProjectDirs;
use tauri::Manager;
use sysinfo::{ProcessExt, System, SystemExt, PidExt};
use winapi::um::winuser::*;

pub mod handlers;

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Project {
    display_name: String,
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
    projects: HashMap<String, Project>,
}

pub fn update_loop(window: tauri::Window, handle: tauri::AppHandle) {
    let mut system = System::new_all();
    thread::spawn(move || loop {
        thread::sleep(UPDATE_INTERVAL);

        system.refresh_all();

        let state = handle.state::<AppState>();
        let mut projects = state.projects.lock().unwrap();

        for project in projects.values_mut() {
            project.open = false;
        }

        for process in system.processes_by_exact_name("Unity.exe") {
            let open_project_names = get_process_windows(process.pid().as_u32()).into_iter()
                .filter(|title| title.contains("- Unity"))
                .map(|title| {
                    let segments: Vec<&str> = title.split("-").collect();

                    if segments.len() == 4 {
                        return segments[0].to_string();
                    }

                    segments[..segments.len() - 3].join("-").to_string()
                });

            for project_name in open_project_names {
                let project = projects
                    .entry(project_name.clone())
                    .or_insert_with(|| Project { 
                        display_name: format_project_name(&project_name), 
                        time: Duration::from_secs(0),
                        open: true,
                    });

                project.time += UPDATE_INTERVAL;
                project.open = true;
            }
        }

        send_update(&projects, &window);
    });
}

fn send_update(projects: &HashMap<String, Project>, window: &tauri::Window) {
    let payload = UpdatePayload {
        projects: projects.clone(),
    };

    window.emit("update", payload).unwrap();
    save(projects);
}

pub fn load() -> HashMap<String, Project> {
    let path = get_data_path();

    if !path.exists() {
        return HashMap::new();
    }

    let contents = fs::read_to_string(path).expect("Failed to read saved projects");
    ron::from_str(&contents).expect("Failed to parse saved projects")
}

pub fn save(map: &HashMap<String, Project>) {
    let path = get_data_path(); 

    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create data directory");
    }

    let contents = ron::to_string(map).expect("Failed to serialize projects");
    fs::write(path, contents).expect("Failed to save projects");
}

fn get_data_path() -> PathBuf {
    let dirs = ProjectDirs::from("com", "Kesomannen", "Time App").unwrap();
    dirs.data_dir().join("projects.ron")
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

fn format_project_name(name: &str) -> String {
    let mut formatted = String::new();
    let mut chars = name.chars();
    let mut last = ' ';

    while let Some(mut c) = chars.next() {
        if c.is_ascii_uppercase() && last.is_ascii_lowercase() {
            formatted.push(' ');
        }

        if c == '-' || c == '_' {
            c = ' ';
        }

        formatted.push(match last {
            ' ' => c.to_ascii_uppercase(),
            _ => c,
        });

        last = c;
    }

    formatted
}