#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::wget,
      cmd::get_minecraft_versions,
      cmd::read_config,
      cmd::write_config,
      cmd::get_default_config,
      cmd::get_instance_list,
      cmd::new_instance
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
