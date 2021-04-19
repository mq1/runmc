#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod config;
mod util;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::list_available_versions,
      cmd::list_versions,
      cmd::install_version,
      cmd::remove_version,
      cmd::run_minecraft,
      cmd::login,
      cmd::accounts,
      cmd::remove_account,
      config::get_default_config,
      config::get_config,
      config::save_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
