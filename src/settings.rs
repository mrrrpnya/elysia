#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    globals::{CONFIG_PATH, DATA_PATH},
    runners::Runner,
    utils::filesystem::ensure_or_default,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalSettings {
    pub wineprefixes_directory: PathBuf,
    pub components_directory: PathBuf,
    pub temp_directory: PathBuf,
    pub cache_directory: PathBuf,

    pub installed_games: HashMap<String, InstalledGame>,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        let data_path = &*DATA_PATH;
        let wineprefixes_directory = data_path.join("wineprefixes/");
        let components_directory = data_path.join("components/");
        let temp_directory = data_path.join("temp/");
        let cache_directory = data_path.join("cache/");

        Self {
            wineprefixes_directory,
            components_directory,
            temp_directory,
            cache_directory,
            installed_games: HashMap::new(),
        }
    }
}

impl GlobalSettings {
    pub fn load() -> Result<GlobalSettings, String> {
        let config_path = &*CONFIG_PATH;
        let exists =
            fs::exists(config_path).map_err(|e| format!("Cannot check if config exists: {e}"))?;
        if !exists {
            return Err("Config does not exist".to_string());
        }

        let data = fs::read(config_path).map_err(|e| format!("Cannot read config file: {e}"))?;
        let mut settings = serde_json::from_slice::<GlobalSettings>(&data)
            .map_err(|e| format!("Cannot deserialize saved config: {e}"))?;

        settings.validate();

        Ok(settings)
    }

    pub fn save(&self) {
        let data = serde_json::to_vec_pretty(self);
        match data {
            Ok(data) => {
                if let Err(e) = fs::write(&*CONFIG_PATH, data) {
                    println!("Error when writing config file: {e}");
                }
            }
            Err(e) => {
                println!("Error when serializing config: {e}");
            }
        }
    }

    pub fn validate(&mut self) {
        let mut check_fn = || -> Result<(), String> {
            self.wineprefixes_directory = ensure_or_default(
                &self.wineprefixes_directory,
                &DATA_PATH.join("wineprefixes"),
            )?
            .to_path_buf();
            self.components_directory =
                ensure_or_default(&self.components_directory, &DATA_PATH.join("components"))?
                    .to_path_buf();
            self.temp_directory =
                ensure_or_default(&self.temp_directory, &DATA_PATH.join("temp"))?.to_path_buf();
            self.cache_directory =
                ensure_or_default(&self.cache_directory, &DATA_PATH.join("cache"))?.to_path_buf();

            Ok(())
        };

        if let Err(e) = check_fn() {
            panic!("Cannot use current or default path: {e}");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledGame {
    pub id: String,
    pub biz_name: String,
    pub install_path: PathBuf,
    pub executable_path: PathBuf,
    pub command_wrapper: Option<String>,
    pub command_arguments: Option<String>,
    pub environment: HashMap<String, String>,
    pub runner: Runner,
    pub runtime_components: Vec<RuntimeComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeComponent {
    Dxvk(ComponentVersion),
    Vkd3dProton(ComponentVersion),
    DxvkNvApi(ComponentVersion),
}

type ComponentVersion = String;
