use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub username: Option<String>,

    #[serde(default = "default_room_id")]
    pub room_id: u32,

    pub phone_number: Option<String>,
    pub license_plate: Option<String>,
}

fn default_room_id() -> u32 {
    10
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            room_id: default_room_id(),
            phone_number: None,
            license_plate: None,
        }
    }
}

impl Config {
    pub fn get_api_url() -> String {
        "https://stregsystem.fklub.dk/api".to_string()
    }

    pub fn get_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| AppError::Config("Could not find home directory".to_string()))?;
        let config_dir = home_dir.join(".config");

        Ok(config_dir.join(".stregsystemet.toml"))
    }

    pub fn ensure_config_dir_exists() -> Result<()> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    AppError::Config(format!("Failed to create config directory: {e}"))
                })?;
            }
        }
        Ok(())
    }

    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {e}")))
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(ref username) = self.username {
            if username.trim().is_empty() {
                return Err(AppError::Config(
                    "Username cannot be empty or whitespace only".to_string(),
                ));
            }
        }

        Ok(())
    }
}
