use std::{
  error::Error,
  fs, io,
  path::{Path, PathBuf},
};

pub fn get_base_dir() -> Result<PathBuf, Box<dyn Error>> {
  let base_path = tauri::api::path::home_dir()
    .ok_or("Cannot find base home dir")?
    .join(".runmc");

  Ok(base_path)
}

pub async fn download_file<S: AsRef<str>, P: AsRef<Path>>(
  url: S,
  path: P,
) -> Result<(), Box<dyn Error>> {
  // create parent dirs
  let parent_dir = path.as_ref().parent().ok_or("cannot retrieve parent dir")?;
  fs::create_dir_all(parent_dir)?;

  // download only if file doesn't already exist
  if !path.as_ref().exists() {
    println!("downloading {} to {:?}", url.as_ref(), path.as_ref());
    let resp = reqwest::get(url.as_ref()).await?.bytes().await?;
    let mut bytes = resp.as_ref();

    let mut out = fs::File::create(path)?;
    io::copy(&mut bytes, &mut out)?;
    println!("file downloaded");
  } else {
    println!("{:?} already present", path.as_ref());
  }

  Ok(())
}

pub fn ls<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
  let entries = tauri::api::dir::read_dir(path, false)?;
  let mut entries = entries.into_iter().map(|entry| entry.name.unwrap()).collect::<Vec<String>>();

  entries.retain(|entry| entry.ne(".DS_Store"));

  Ok(entries)
}
