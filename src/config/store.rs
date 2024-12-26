use std::fs;

use crate::config::settings::Config;
use crate::error::{AppError, Result};

pub fn load_or_create_config() -> Result<Config> {
    let config_path = Config::get_config_path()?;

    if !config_path.exists() {
        Config::ensure_config_dir_exists()?;

        let default_config = Config::default();
        fs::write(&config_path, default_config.to_toml_string()?)?;

        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| AppError::Config(format!("Failed to parse config: {e}")))?;

    config.validate()?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    config.validate()?;

    Config::ensure_config_dir_exists()?;

    let config_path = Config::get_config_path()?;
    fs::write(&config_path, config.to_toml_string()?)?;

    Ok(())
}
