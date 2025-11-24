use crate::error::DisplayError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    // Map of Display Name (e.g., "\\.\DISPLAY1") to a list of (VCP Code, Value)
    pub settings: HashMap<String, Vec<(u8, u32)>>,
}

impl Config {
    pub fn load() -> Result<Self, DisplayError> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), DisplayError> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, DisplayError> {
        let proj_dirs = ProjectDirs::from("com", "dispman", "dispman")
            .ok_or_else(|| DisplayError::ConfigError("Could not determine config directory".to_string()))?;
        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn save_profile(&mut self, name: String, profile: Profile) {
        self.profiles.insert(name, profile);
    }
}
