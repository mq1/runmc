use tauri::command;
use std::{fs::File, io};

#[command]
pub fn wget(url: String, path: String) -> Result<(), String> {
  let resp = ureq::get(&url).call().expect("request failed");
  let mut reader = resp.into_reader();
  let mut writer = File::create(path).expect("failed to create file");
  io::copy(&mut reader, &mut writer).expect("failed to write file");

  Ok(())
}

// launchermeta

#[command]
pub fn get_minecraft_versions() -> Result<Vec<libmc::launchermeta::Version>, String> {
  libmc::launchermeta::get_minecraft_versions().map_err(|e| e.to_string())
}

// config

#[command]
pub fn read_config() -> Result<libmc::config::Config, String> {
  libmc::config::read().map_err(|e| e.to_string())
}

#[command]
pub fn write_config(config: libmc::config::Config) -> Result<(), String> {
  libmc::config::write(&config).map_err(|e| e.to_string())
}

#[command]
pub fn get_default_config() -> libmc::config::Config {
  libmc::config::get_default_config()
}

// instances

#[command]
pub fn get_instance_list() -> Result<Vec<String>, String> {
  libmc::instances::get_instance_list().map_err(|e| e.to_string())
}

#[command]
pub fn new_instance(name: String, minecraft_version: String, minecraft_version_manifest_url: String) -> Result<(), String> {
  libmc::instances::new_instance(&name, &minecraft_version, &minecraft_version_manifest_url).map_err(|e| e.to_string())
}
