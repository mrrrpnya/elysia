#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::runners::Runner;

// FIXME: use default values if saved config is missing fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    #[serde(skip)]
    pub data_path: PathBuf,

    #[serde(skip)]
    pub config_path: PathBuf,

    pub wineprefixes_directory: PathBuf,
    pub components_directory: PathBuf,
    pub temp_directory: PathBuf,
    pub cache_directory: PathBuf,

    pub installed_games: HashMap<String, InstalledGame>,
}

impl GlobalSettings {
    pub fn default(data_path: &Path) -> Self {
        let wineprefixes_directory = data_path.join("wineprefixes/");
        let components_directory = data_path.join("components/");
        let temp_directory = data_path.join("temp/");
        let cache_directory = data_path.join("cache/");

        let check_fn = || -> Result<(), String> {
            Self::ensure_dir(&wineprefixes_directory)?;
            Self::ensure_dir(&components_directory)?;
            Self::ensure_dir(&temp_directory)?;
            Self::ensure_dir(&cache_directory)?;
            Ok(())
        };

        if let Err(e) = check_fn() {
            panic!("Failed to create directory: {e}")
        }

        Self {
            data_path: data_path.to_owned(),
            config_path: data_path.join("config.json"),
            wineprefixes_directory,
            components_directory,
            temp_directory,
            cache_directory,
            installed_games: HashMap::new(),
        }
    }

    pub fn load(data_path: &Path) -> Result<GlobalSettings, String> {
        let config_path = data_path.join("config.json");
        let exists =
            fs::exists(&config_path).map_err(|e| format!("Cannot check if config exists: {e}"))?;
        if !exists {
            return Err("Config does not exist".to_string());
        }

        let data = fs::read(&config_path).map_err(|e| format!("Cannot read config file: {e}"))?;
        let mut settings = serde_json::from_slice::<GlobalSettings>(&data)
            .map_err(|e| format!("Cannot deserialize saved config: {e}"))?;

        settings.data_path = data_path.to_owned();
        settings.config_path = config_path;

        settings.validate();

        Ok(settings)
    }

    pub fn save(&self) {
        let data = serde_json::to_vec(self);
        match data {
            Ok(data) => {
                if let Err(e) = fs::write(&self.config_path, data) {
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
            if let Some(path) = Self::exists_or_default(
                &self.wineprefixes_directory,
                &self.data_path,
                "wineprefixes",
            )? {
                self.wineprefixes_directory = path;
            };
            if let Some(path) =
                Self::exists_or_default(&self.components_directory, &self.data_path, "components")?
            {
                self.components_directory = path;
            };
            if let Some(path) =
                Self::exists_or_default(&self.temp_directory, &self.data_path, "temp")?
            {
                self.temp_directory = path;
            };
            if let Some(path) =
                Self::exists_or_default(&self.cache_directory, &self.data_path, "cache")?
            {
                self.cache_directory = path;
            };

            Ok(())
        };

        if let Err(e) = check_fn() {
            panic!("Cannot use current or default path: {e}");
        }
    }

    fn ensure_dir(dir: &Path) -> Result<(), String> {
        let exists = fs::exists(dir)
            .map_err(|e| format!("Cannot check if directory {dir:?} exists: {e}"))?;
        if exists {
            return Ok(());
        }

        fs::create_dir(dir).map_err(|e| format!("Cannot create directory {dir:?} : {e}"))?;

        Ok(())
    }

    fn exists_or_default(
        dir: &Path,
        data_path: &Path,
        default: &str,
    ) -> Result<Option<PathBuf>, String> {
        match Self::ensure_dir(dir) {
            Ok(()) => Ok(None),
            Err(e) => {
                println!(
                    "Error when creating loading directory from config, will use default: {e}"
                );

                let default = data_path.join(default);
                Self::ensure_dir(&default)?;

                Ok(Some(default))
            }
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
