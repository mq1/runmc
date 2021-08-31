#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{io, fs::File};

#[tauri::command]
async fn wget(url: String, path: String) -> Result<(), String> {
  let resp = reqwest::get(url).await.expect("request failed");
  let bytes = resp.bytes().await.expect("failed reading content");
  let mut bytes = bytes.as_ref();
  let mut out = File::create(path).expect("failed to create file");

  io::copy(&mut bytes, &mut out).expect("failed to copy content");

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![wget])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
