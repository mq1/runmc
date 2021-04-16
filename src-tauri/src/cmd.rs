use serde::{Deserialize, Serialize};
use std::{fs, io, process};
use tauri::command;

#[derive(Serialize, Deserialize)]
pub struct Version {
  id: String,
  r#type: String,
  url: String,
}

#[command]
pub async fn list_available_versions() -> Result<Vec<Version>, String> {
  #[derive(Deserialize)]
  struct Json {
    versions: Vec<Version>,
  }

  let res = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
    .await
    .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;

  Ok(j.versions)
}

#[command]
pub fn list_versions() -> Result<Vec<String>, String> {
  let base_path = tauri::api::path::home_dir()
    .ok_or("Cannot find base home dir")?
    .join(".runmc");
  let path = base_path.join("versions");

  // TODO this can be done with a collect
  // https://doc.rust-lang.org/std/fs/fn.read_dir.html
  let mut versions: Vec<String> = Vec::new();
  for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
    versions.push(String::from(
      entry
        .map_err(|e| e.to_string())?
        .file_name()
        .to_str()
        .ok_or("Error getting file name")?,
    ))
  }

  Ok(versions)
}

#[command]
pub async fn download_file(url: String, path: String) {
  let base_path = tauri::api::path::home_dir().unwrap().join(".runmc");
  let path = base_path.join(path);

  // create parent dirs
  let parent_dir = path.parent().unwrap();
  fs::create_dir_all(parent_dir).unwrap();

  // download only if file doesn't already exist
  if !path.exists() {
    println!("downloading {} to {:?}", url, path);
    let resp = reqwest::get(url)
      .await
      .expect("File download failed")
      .bytes()
      .await
      .unwrap();
    let mut bytes = resp.as_ref();

    let mut out = fs::File::create(path).expect("File creation failed");
    io::copy(&mut bytes, &mut out).expect("File writing failed");
    println!("file downloaded");
  } else {
    println!("{:?} already present", path);
  }
}

#[command]
pub fn remove_version(version: String) -> Result<(), String> {
  let base_path = tauri::api::path::home_dir().ok_or("Cannot find home dir")?.join(".runmc");
  let path = base_path.join(&version);
  fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
  println!("deleted {:?}", &path);

  Ok(())
}

#[command]
pub fn run_minecraft(version: String, access_token: String) {
  let base_path = tauri::api::path::home_dir().unwrap().join(".runmc");
  let path = base_path.join("versions").join(&version);

  println!("launching version {}", &version);

  process::Command::new("java")
    .current_dir(path)
    .arg("-Dminecraft.launcher.brand=runmc")
    .arg("-Xmx2G")
    .arg("-Xms2G")
    .arg("-cp")
    .arg("libraries/*")
    .arg("net.minecraft.client.main.Main")
    .arg("--gameDir")
    .arg("game-data")
    .arg("--assetsDir")
    .arg("assets")
    .arg("--accessToken")
    .arg(&access_token)
    .arg("--version")
    .arg(&version)
    .stdout(process::Stdio::inherit())
    .stderr(process::Stdio::inherit())
    .spawn()
    .expect("error launching minecraft");
}
