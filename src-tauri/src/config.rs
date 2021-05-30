use crate::util::get_base_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::command;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaConfig {
  pub path: String,
  pub memory: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub client_id: String,
  pub java: JavaConfig,
  pub locale: String,
}

#[command]
pub fn get_default_config() -> Config {
  Config {
    client_id: Uuid::new_v4().to_string(),
    java: JavaConfig {
      path: String::from("javaw"),
      memory: String::from("2G"),
    },
    locale: String::from("en"),
  }
}

#[command]
pub fn get_config() -> Result<Config, String> {
  let path = get_base_dir()?.join("config.yaml");
  let config: Config;

  if !path.exists() {
    // write the default config if not found
    config = get_default_config();
    let text = serde_yaml::to_string(&config).map_err(|e| e.to_string())?;
    fs::write(&path, &text).map_err(|e| e.to_string())?;
  } else {
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    config = serde_yaml::from_str(&text).map_err(|e| e.to_string())?;
  }

  Ok(config)
}

#[command]
pub fn save_config(config: Config) -> Result<(), String> {
  let path = get_base_dir()?.join("config.yaml");
  let text = serde_yaml::to_string(&config).map_err(|e| e.to_string())?;
  fs::write(&path, &text).map_err(|e| e.to_string())?;

  Ok(())
}
