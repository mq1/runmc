use crate::{account, config, fabric, instance, version};
use tauri::command;

// CONFIG

#[command]
pub fn get_default_config() -> config::Config {
  config::get_defaults()
}

#[command]
pub fn get_config() -> Result<config::Config, String> {
  config::read().map_err(|e| e.to_string())
}

#[command]
pub fn save_config(config: config::Config) -> Result<(), String> {
  config::save(config).map_err(|e| e.to_string())
}

// FABRIC

#[command]
pub async fn get_fabric_loader_versions(
  minecraft_version: String,
) -> Result<Vec<fabric::Loader>, String> {
  fabric::get_loader_versions(minecraft_version)
    .await
    .map_err(|e| e.to_string())
}

#[command]
pub async fn install_fabric(instance_name: String, loader_version: String) -> Result<(), String> {
  fabric::install(instance_name, loader_version)
    .await
    .map_err(|e| e.to_string())
}

// INSTANCE

#[command]
pub fn list_instances() -> Result<Vec<String>, String> {
  instance::list().map_err(|e| e.to_string())
}

#[command]
pub async fn new_instance(instance_name: String, minecraft_version: version::MinecraftVersion) -> Result<(), String> {
  instance::new(instance_name, minecraft_version)
    .await
    .map_err(|e| e.to_string())
}

#[command]
pub fn read_instance_info(instance_name: String) -> Result<instance::InstanceInfo, String> {
  instance::read_info(instance_name).map_err(|e| e.to_string())
}

#[command]
pub fn rename_instance(current_name: String, new_name: String) -> Result<(), String> {
  instance::rename(current_name, new_name).map_err(|e| e.to_string())
}

#[command]
pub fn delete_instance(name: String) -> Result<(), String> {
  instance::delete(name).map_err(|e| e.to_string())
}

#[command]
pub fn list_instance_mods(instance_name: String) -> Result<Vec<String>, String> {
  instance::list_mods(instance_name).map_err(|e| e.to_string())
}

#[command]
pub fn open_instance_dir(instance_name: String) -> Result<(), String> {
  instance::open_dir(instance_name).map_err(|e| e.to_string())
}

#[command]
pub fn open_instance_mods_dir(instance_name: String) -> Result<(), String> {
  instance::open_mods_dir(instance_name).map_err(|e| e.to_string())
}

#[command]
pub async fn run_instance(instance_name: String, account: account::Account) -> Result<(), String> {
  instance::run(instance_name, account)
    .await
    .map_err(|e| e.to_string())
}

// VERSION

#[command]
pub async fn list_available_minecraft_versions() -> Result<Vec<version::MinecraftVersion>, String> {
  version::list_available().await.map_err(|e| e.to_string())
}
