use serde::Deserialize;
use directories::ProjectDirs;
use std::{fs, path::PathBuf};
use std::error::Error;

const DEFAULT_CONFIG: &str = include_str!("config.toml");

#[derive(Debug, Deserialize)]
pub struct Config {
    pub decorate: DecorateConfig,
}

#[derive(Debug, Deserialize)]
pub struct DecorateConfig {
    pub properties: PropertyColors,
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize)]
pub struct PropertyColors {
    pub hydrophobic: String,
    pub positive: String,
    pub negative: String,
    pub polar: String,
    pub special: String,
}

#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    pub line_width: usize,
    // pub show_index: bool,
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let proj_dirs = ProjectDirs::from("org", "whey", "whey")
        .ok_or("Could not locate config directory")?;

    let config_path: PathBuf = proj_dirs.config_dir().join("config.toml");

    // Defaults
    let mut config: Config = toml::from_str(DEFAULT_CONFIG)?;

    // Se o usuário tiver config
    if config_path.exists() {
        let user_cfg = fs::read_to_string(&config_path)?;
        config = toml::from_str(&user_cfg)?;
    }

    Ok(config)
}
