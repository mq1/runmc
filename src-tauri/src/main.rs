#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;

mod cmd;

mod account;
mod config;
mod fabric;
mod instance;
mod util;
mod version;

fn main() {
  let base_dir = util::get_base_dir().expect("Error getting base dir");
  fs::create_dir_all(base_dir).expect("Error creating base dir");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::list_accounts,
      cmd::remove_account,
      cmd::login,
      cmd::get_default_config,
      cmd::get_config,
      cmd::save_config,
      cmd::get_fabric_loader_versions,
      cmd::install_fabric,
      cmd::list_instances,
      cmd::new_instance,
      cmd::read_instance_info,
      cmd::rename_instance,
      cmd::delete_instance,
      cmd::list_instance_mods,
      cmd::open_instance_dir,
      cmd::open_instance_mods_dir,
      cmd::run_instance,
      cmd::list_available_minecraft_versions
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
