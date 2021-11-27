#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::wget,
      cmd::get_instance_list,
      cmd::read_config,
      cmd::write_config,
      cmd::get_default_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
