use std::{path::PathBuf, collections::HashMap, fs, io, env};

use directories::ProjectDirs;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use super::*;

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
    let dirs = ProjectDirs::from("com", "Kesomannen", PRODUCT_NAME).unwrap();
    dirs.data_dir().join("projects.ron")
}

pub fn set_auto_startup(enabled: bool) -> io::Result<()> {
    println!("Setting auto startup to {}", enabled);
    let path = get_exe_path()?;
    let key = get_auto_startup_key()?;
    if enabled {
        key.set_value(PRODUCT_NAME, &path)?;
    } else {
        key.delete_value(PRODUCT_NAME)?;
    }
    Ok(())
}

pub fn get_auto_startup() -> io::Result<bool> {
    let path = get_exe_path()?;
    let reg = get_auto_startup_key()?;
    let value = reg.get_value::<String, _>(PRODUCT_NAME).unwrap_or_default();
    Ok(value == path)
}

fn get_auto_startup_key() -> io::Result<RegKey> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let reg = hklm.open_subkey(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"#)?;
    Ok(reg)
}

fn get_exe_path() -> io::Result<String> {
    let path = env::current_exe()?.to_string_lossy().to_string();
    Ok(path)
}