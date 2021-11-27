use tauri::command;
use std::{io, fs::File};

#[command]
pub async fn wget(url: String, path: String) -> Result<(), String> {
  let resp = reqwest::get(url).await.expect("request failed");
  let bytes = resp.bytes().await.expect("failed reading content");
  let mut bytes = bytes.as_ref();
  let mut out = File::create(path).expect("failed to create file");

  io::copy(&mut bytes, &mut out).expect("failed to copy content");

  Ok(())
}

// launchermeta

#[command]
pub fn get_instance_list() -> Result<Vec<String>, String> {
  libmc::instances::get_instance_list().map_err(|e| e.to_string())
}

// config

#[command]
pub fn read_config() -> Result<libmc::config::Config, String> {
  libmc::config::read().map_err(|e| e.to_string())
}

#[command]
pub fn write_config(config: libmc::config::Config) -> Result<(), String> {
  libmc::config::write(config).map_err(|e| e.to_string())
}

#[command]
pub fn get_default_config() -> Result<libmc::config::Config, String> {
  libmc::config::get_default_config().map_err(|e| e.to_string())
}
