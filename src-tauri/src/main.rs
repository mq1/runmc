#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod account;
mod config;
mod fabric;
mod instance;
mod util;
mod version;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      version::list_available_game_versions,
      instance::get_instance_info,
      instance::init_instance,
      instance::list_instances,
      instance::rename_instance,
      instance::remove_instance,
      instance::run_instance,
      instance::install_fabric,
      instance::list_mods,
      instance::open_instance_dir,
      instance::open_mods_dir,
      account::login,
      account::get_accounts,
      account::remove_account,
      config::get_default_config,
      config::get_config,
      config::save_config,
      fabric::get_fabric_loader_versions,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
