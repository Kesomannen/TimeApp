use std::{collections::HashMap, sync::{Mutex, MutexGuard}};

use crate::{persistent::save_config, PRODUCT_NAME};

use auto_launch::AutoLaunch;
use serde::{de::DeserializeOwned, Serialize};

pub struct Options {
    map: Mutex<HashMap<String, String>>,
    auto_launch: AutoLaunch,
}

impl Options {
    pub fn new(map: HashMap<String, String>) -> Self {
        let path = std::env::current_dir().unwrap();
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
    
        match map.get(key) {
            Some(val) => ron::from_str(&val).unwrap(),
            None => {
                let val = ron::to_string(&default).unwrap();
                map.insert(key.to_string(), val.clone());
                save_config(&map);
                default
            }
        }
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
                    self.auto_launch.enable().unwrap();
                } else {
                    self.auto_launch.disable().unwrap();
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