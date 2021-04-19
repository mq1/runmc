use crate::util::get_base_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::command;

#[derive(Deserialize, Serialize)]
pub struct Config {
  pub java_path: String,
  pub java_memory_mb: i32,
}

fn get_default_config() -> Config {
  Config {
    java_path: String::from("java"),
    java_memory_mb: 2048,
  }
}

#[command]
pub fn get_config() -> Result<Config, String> {
  let path = get_base_dir()?.join("config.json");
  let config: Config;

  if !path.exists() {
    // write the default config if not found
    config = get_default_config();
    let text = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    fs::write(&path, &text).map_err(|e| e.to_string())?;
  } else {
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    config = serde_json::from_str(&text).map_err(|e| e.to_string())?;
  }

  Ok(config)
}

#[command]
pub fn save_config(config: Config) -> Result<(), String> {
  let path = get_base_dir()?.join("config.json");
  let text = serde_json::to_string(&config).map_err(|e| e.to_string())?;
  fs::write(&path, &text).map_err(|e| e.to_string())?;

  Ok(())
}
