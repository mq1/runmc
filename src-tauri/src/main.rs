#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;

mod cmd;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::wget
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
