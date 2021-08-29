use std::io;
use std::fs::File;
use tauri::command;

#[command]
pub async fn wget(url: String, path: String) -> Result<(), String> {
  let resp = reqwest::get(url).await.expect("request failed");
  let bytes = resp.bytes().await.expect("failed reading content");
  let mut bytes = bytes.as_ref();
  let mut out = File::create(path).expect("failed to create file");

  io::copy(&mut bytes, &mut out).expect("failed to copy content");

  Ok(())
}
