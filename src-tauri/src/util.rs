use std::{fs, io, path::PathBuf};

pub fn get_base_dir() -> Result<PathBuf, String> {
  let base_path = tauri::api::path::home_dir()
    .ok_or("Cannot find base home dir")?
    .join(".runmc");

  Ok(base_path)
}

pub async fn download_file(url: String, path: PathBuf) -> Result<(), String> {
  // create parent dirs
  let parent_dir = path.parent().ok_or("cannot retrieve parent dir")?;
  fs::create_dir_all(parent_dir).map_err(|e| e.to_string())?;

  // download only if file doesn't already exist
  if !path.exists() {
    println!("downloading {} to {:?}", url, path);
    let resp = reqwest::get(url)
      .await
      .map_err(|e| e.to_string())?
      .bytes()
      .await
      .map_err(|e| e.to_string())?;
    let mut bytes = resp.as_ref();

    let mut out = fs::File::create(path).map_err(|e| e.to_string())?;
    io::copy(&mut bytes, &mut out).map_err(|e| e.to_string())?;
    println!("file downloaded");
  } else {
    println!("{:?} already present", path);
  }

  Ok(())
}
