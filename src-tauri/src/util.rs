use std::path::PathBuf;

pub fn get_base_dir() -> Result<PathBuf, String> {
  let base_path = tauri::api::path::home_dir()
    .ok_or("Cannot find base home dir")?
    .join(".runmc");

  Ok(base_path)
}
