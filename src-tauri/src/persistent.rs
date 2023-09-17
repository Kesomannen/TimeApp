use std::{path::PathBuf, collections::HashMap, fs, io};

use directories::ProjectDirs;
use serde::de::DeserializeOwned;

use super::*;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Ron(ron::Error),
    RonSpanned(ron::error::SpannedError),
}

type Result<T> = std::result::Result<T, Error>;

pub fn load_projects() -> HashMap<String, Project> {
    read_from(data_path()).expect("Failed to load projects")
}

pub fn save_projects(map: &HashMap<String, Project>) {
    write_to(data_path(), map).expect("Failed to save projects");
}

pub fn load_config() -> HashMap<String, String> {
    read_from(config_path()).expect("Failed to load config")
}

pub fn save_config(map: &HashMap<String, String>) {
    write_to(config_path(), map).expect("Failed to save config");
}

fn read_from<T>(path: PathBuf) -> Result<T> where T: Default + DeserializeOwned {
    Ok(match fs::read_to_string(path) {
        Ok(c) => ron::from_str(&c).map_err(Error::RonSpanned)?,
        Err(_) => T::default(),
    })
}

fn write_to<T>(path: PathBuf, value: &T) -> Result<()> where T: serde::Serialize {
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).map_err(Error::Io)?;
    }

    let contents = ron::to_string(value).map_err(Error::Ron)?;
    fs::write(path, contents).map_err(Error::Io)?;

    Ok(())
}

fn data_path() -> PathBuf {
    project_dirs().data_dir().join("projects.ron")
}

fn config_path() -> PathBuf {
    project_dirs().config_dir().join("config.ron")
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "Kesomannen", PRODUCT_NAME).unwrap()
}
