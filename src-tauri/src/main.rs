#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod account;
mod cmd;
mod config;
mod fabric;
mod util;
mod version;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      version::list_available_versions,
      version::list_versions,
      version::install_version,
      version::rename_version,
      version::remove_version,
      cmd::run_minecraft,
      account::login,
      account::accounts,
      account::remove_account,
      config::get_default_config,
      config::get_config,
      config::save_config,
      fabric::get_fabric_loader_versions,
      fabric::install_fabric,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
