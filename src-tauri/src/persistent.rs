use std::{path::PathBuf, collections::HashMap, fs};

use directories::ProjectDirs;

use super::Project;

pub fn load_projects() -> HashMap<String, Project> {
    let path = get_data_path();

    if !path.exists() {
        return HashMap::new();
    }

    let contents = fs::read_to_string(path).expect("Failed to read saved projects");
    ron::from_str(&contents).expect("Failed to parse saved projects")
}

pub fn save_projects(map: &HashMap<String, Project>) {
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
