use crate::err;
use crate::interaction::Coordinates;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Strategy {
    Local,
    Remote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub default_strategy: Strategy,

    pub instructions: Instructions,
    pub remote: Option<Remote>,

    pub sleep_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Remote {
    pub url: String,
    pub max_retries: u8,
    pub timeout_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Instructions {
    pub unlock_chest: Coordinates,
    pub character_switch: Coordinates,
}

const CONFIG_FILE_NAME: &str = "config.toml";

pub fn dir() -> PathBuf {
    ProjectDirs::from("net", "liefland", "idle-champions-redeemer")
        .unwrap()
        .config_dir()
        .to_path_buf()
}

pub fn file() -> std::path::PathBuf {
    dir().join(CONFIG_FILE_NAME)
}

pub fn read() -> Result<ConfigFile, &'static str> {
    let path = file();
    if !path.exists() {
        return Err("Config file does not exist");
    }

    let tml = std::fs::read_to_string(path).map_err(|_| "Failed to read config file")?;

    toml::from_str(&tml).map_err(|_| "Failed to parse config file")
}

pub fn write(config: &ConfigFile) -> Result<(), &'static str> {
    let path = file();

    let tml = toml::to_string(&config).map_err(|_| "Failed to serialize config file")?;

    match backup() {
        Ok(_) => {}
        Err(_) => {
            err!("Failed to backup config file");
        }
    }

    std::fs::write(path, tml).map_err(|_| "Failed to write config file")?;

    Ok(())
}

pub fn remove() -> Result<(), &'static str> {
    let path = file();

    if !path.exists() {
        return Ok(());
    }

    std::fs::remove_file(path).map_err(|_| "Failed to remove config file")?;

    Ok(())
}

fn backup() -> Result<(), std::io::Error> {
    let config_path = dir();
    let file_name = config_path.join(CONFIG_FILE_NAME);
    let backup_file_name = config_path.join(format!("{}.bak", CONFIG_FILE_NAME));

    if !file_name.exists() {
        return Ok(());
    }

    if backup_file_name.exists() {
        std::fs::remove_file(&backup_file_name)?;
    }

    std::fs::rename(file_name, backup_file_name)?;

    Ok(())
}
