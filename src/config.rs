use serde::{Deserialize, Serialize};
use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub anki2_path: Option<String>,
    pub default_collection: Option<String>,
}

pub fn get_config_path() -> PathBuf {
    dirs::config_dir().unwrap().join("rust_forvo_api").join("config.json")
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let config_path = get_config_path();
    fs::create_dir_all(config_path.parent().unwrap())?;
    let config_data = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(config_path)?;
    writeln!(file, "{}", config_data)?;
    Ok(())
}

pub fn load_config() -> io::Result<Config> {
    let config_path = get_config_path();
    if config_path.exists() {
        let config_data = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

