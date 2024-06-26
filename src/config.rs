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

    pub slow: bool,
}

#[allow(dead_code)] // Can be dead code if the feature is not enabled
#[derive(Debug, Serialize, Deserialize)]
pub struct Remote {
    pub url: Option<String>,
    pub max_retries: u8,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Instructions {
    pub unlock_chest: Coordinates,
}

const CONFIG_FILE_NAME: &str = "config.json";

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

    let jsn = std::fs::read_to_string(path).map_err(|_| "Failed to read config file")?;

    serde_json::from_str(&jsn).map_err(|_| "Failed to parse config file")
}

pub fn write(config: &ConfigFile) -> Result<(), &'static str> {
    let path = file();

    let jsn = serde_json::to_string(&config).map_err(|_| "Failed to serialize config file")?;

    match backup() {
        Ok(_) => {}
        Err(_) => {
            err!("Failed to backup config file");
        }
    }

    std::fs::write(path, jsn).map_err(|_| "Failed to write config file")?;

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

pub fn change_default_strategy() -> Result<(), &'static str> {
    let mut config = read()?;

    match config.default_strategy {
        Strategy::Local => {
            config.default_strategy = Strategy::Remote;
            println!("Default Strategy changed to Remote");
        }
        Strategy::Remote => {
            config.default_strategy = Strategy::Local;
            println!("Default Strategy changed to Local");
        }
    }

    write(&config)
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
