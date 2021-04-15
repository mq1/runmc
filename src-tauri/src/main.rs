#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::list_versions,
      cmd::download_file,
      cmd::remove_dir,
      cmd::run_minecraft
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
