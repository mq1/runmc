use crate::account::Account;
use crate::config::get_config;
use crate::util::get_base_dir;
use std::{fs, io, path::Path, process};
use tauri::command;
use serde::Deserialize;

#[command]
pub fn run_minecraft(version: String, account: Account) -> Result<(), String> {
  let path = get_base_dir()?.join("versions").join(&version);
  let config = get_config()?;

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Json {
    main_class: String,
  }
  let version_info = fs::read_to_string(path.join("info.json")).map_err(|e| e.to_string())?;
  let version_info: Json = serde_json::from_str(&version_info).map_err(|e| e.to_string())?;

  // get asset index
  let entries = fs::read_dir(&path.join("assets").join("indexes"))
    .map_err(|e| e.to_string())?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()
    .map_err(|e| e.to_string())?;
  let asset_index = entries[0].file_name().ok_or("asset index not found")?;
  let asset_index = Path::new(asset_index)
    .file_stem()
    .ok_or("error getting asset index file name")?;

  println!("launching version {}", &version);

  process::Command::new(&config.java_path)
    .current_dir(&path)
    .arg("-Dminecraft.launcher.brand=runmc")
    .arg(format!("-Xmx{}M", &config.java_memory_mb))
    .arg(format!("-Xms{}M", &config.java_memory_mb))
    .arg("-cp")
    .arg("libraries/*")
    .arg(&version_info.main_class)
    .arg("--gameDir")
    .arg("game-data")
    .arg("--assetsDir")
    .arg("assets")
    .arg("--assetIndex")
    .arg(&asset_index)
    .arg("--username")
    .arg(&account.name)
    .arg("--uuid")
    .arg(&account.id)
    .arg("--accessToken")
    .arg(&account.access_token)
    .arg("--version")
    .arg(&version)
    .stdout(process::Stdio::inherit())
    .stderr(process::Stdio::inherit())
    .spawn()
    .map_err(|e| e.to_string())?;

  Ok(())
}
