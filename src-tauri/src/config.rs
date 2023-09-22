use std::{collections::HashMap, sync::{Mutex, MutexGuard}, path::PathBuf};

use crate::persistent::save_config;

use auto_launch::AutoLaunch;
use serde::{de::DeserializeOwned, Serialize};

pub struct Options {
    map: Mutex<HashMap<String, String>>,
    auto_launch: AutoLaunch,
}

impl Options {
    pub fn new(map: HashMap<String, String>) -> Self {
        let path = PathBuf::from(r#"C:\Users\bobbo\Documents\time_app\src-tauri\target\debug\UnityDevTimer.exe"#);
        let name = path.file_name().unwrap().to_str().unwrap();
        let path = path.to_str().unwrap();

        let auto_launch = AutoLaunch::new(name, path, &[] as &[&str]);

        let config = Self {
            auto_launch,
            map: Mutex::new(map),
        };

        config.update_all();

        config
    }

    pub fn get_raw_key(&self, key: &str) -> Option<String> {
        self.get_map().get(key).cloned()
    }
    
    pub fn get_key_or<T>(&self, key: &str, default: T) -> T where T: Serialize + DeserializeOwned + Clone {
        let mut map = self.get_map();
        let value = map.get(key);

        if let Some(val) = value {
            if let Ok(val) = ron::from_str(&val) {
                return val;
            }
        }

        let val = ron::to_string(&default).unwrap();
        map.insert(key.to_string(), val.clone());
        save_config(&map);
        default
    }
    
    pub fn set_raw_key(&self, key: String, value: String) {
        let mut map = self.get_map();
        self.update(&key, &value);
        map.insert(key, value);
        save_config(&map);
    }
    
    pub fn set_key<T>(&self, key: String, value: T) where T: Serialize + DeserializeOwned {
        self.set_raw_key(key, ron::to_string(&value).unwrap());
    }

    fn update(&self, key: &str, value: &str) {
        match key {
            "auto_start" => {
                if value == "true" {
                    self.auto_launch.enable().unwrap_or_else(|err| {
                        eprintln!("Error enabling auto launch: {}", err);
                    });
                } else {
                    self.auto_launch.disable().unwrap_or_else(|err| {
                        eprintln!("Error disabling auto launch: {}", err);
                    });
                }
            },
            _ => { }
        }
    }

    fn update_all(&self) {
        let map = self.map.lock().unwrap();

        for (key, value) in map.iter() {
            self.update(key, value);
        }
    }

    fn get_map(&self) -> MutexGuard<HashMap<String, String>> {
        self.map.lock().unwrap()
    }
}