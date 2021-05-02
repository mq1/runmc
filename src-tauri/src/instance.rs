use crate::account::Account;
use crate::config::get_config;
use crate::fabric::download_fabric;
use crate::util::get_base_dir;
use crate::version::{download_version, GameVersion};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path, process};
use tauri::command;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
  pub game_version: String,
  pub main_class: String,
  pub fabric: bool,
}

#[command]
pub fn get_instance_info(instance_name: String) -> Result<InstanceInfo, String> {
  let path = get_base_dir()?
    .join("instances")
    .join(instance_name)
    .join("info.yaml");

  let info = fs::read_to_string(path).map_err(|e| e.to_string())?;
  let info: InstanceInfo = serde_yaml::from_str(&info).map_err(|e| e.to_string())?;

  Ok(info)
}

pub fn save_instance_info(instance_name: &String, info: &InstanceInfo) -> Result<(), String> {
  let path = get_base_dir()?
    .join("instances")
    .join(instance_name)
    .join("info.yaml");

  let text = serde_yaml::to_string(info).map_err(|e| e.to_string())?;
  fs::write(&path, &text).map_err(|e| e.to_string())?;

  Ok(())
}

#[command]
pub fn list_instances() -> Result<Vec<String>, String> {
  let path = get_base_dir()?.join("instances");

  // TODO this can be done with a collect
  // https://doc.rust-lang.org/std/fs/fn.read_dir.html
  let mut instances: Vec<String> = Vec::new();
  for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
    instances.push(String::from(
      entry
        .map_err(|e| e.to_string())?
        .file_name()
        .to_str()
        .ok_or("Error getting file name")?,
    ))
  }

  Ok(instances)
}

#[command]
pub async fn init_instance(instance_name: String, game_version: GameVersion) -> Result<(), String> {
  let dir = get_base_dir()?.join("instances").join(&instance_name);
  let main_class = download_version(&dir, &game_version).await?;

  let instance_info = InstanceInfo {
    game_version: game_version.id,
    main_class: main_class,
    fabric: false,
  };

  save_instance_info(&instance_name, &instance_info)?;

  // game-data is the .minecraft directory (kinda)
  fs::create_dir(dir.join("game-data")).map_err(|e| e.to_string())?;

  println!("instance created");
  Ok(())
}

#[command]
pub fn rename_instance(current_name: String, new_name: String) -> Result<(), String> {
  let instances_dir = get_base_dir()?.join("instances");

  fs::rename(
    instances_dir.join(&current_name),
    instances_dir.join(&new_name),
  )
  .map_err(|e| e.to_string())?;

  Ok(())
}

#[command]
pub fn remove_instance(name: String) -> Result<(), String> {
  let path = get_base_dir()?.join("instances").join(&name);
  fs::remove_dir_all(&path).map_err(|e| e.to_string())?;

  println!("deleted {:?}", &path);
  Ok(())
}

#[command]
pub async fn install_fabric(instance_name: String, loader_version: String) -> Result<(), String> {
  let dir = get_base_dir()?.join("instances").join(&instance_name);
  let mut info = get_instance_info(String::from(&instance_name))?;
  let main_class = download_fabric(&dir, &info.game_version, &loader_version).await?;

  // new info
  info.main_class = main_class;
  info.fabric = true;

  save_instance_info(&instance_name, &info)?;

  println!("\nfabric installed");
  Ok(())
}

#[command]
pub fn list_mods(instance_name: String) -> Result<Vec<String>, String> {
  let dir = get_base_dir()?
    .join("instances")
    .join(&instance_name)
    .join("game-data")
    .join("mods");

  let mods = fs::read_dir(dir)
    .map_err(|e| e.to_string())?
    .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
    .collect::<Result<Vec<String>, io::Error>>()
    .map_err(|e| e.to_string())?;

  Ok(mods)
}

#[command]
pub fn open_mods_dir(instance_name: String) -> Result<(), String> {
  let dir = get_base_dir()?
    .join("instances")
    .join(&instance_name)
    .join("game-data")
    .join("mods");

  tauri::api::shell::open(String::from(dir.to_str().unwrap()), None).map_err(|e| e.to_string())?;

  Ok(())
}

#[command]
pub fn run_instance(instance: String, account: Account) -> Result<(), String> {
  let path = get_base_dir()?.join("instances").join(&instance);
  let config = get_config()?;

  let instance_info = get_instance_info(String::from(&instance))?;

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

  // cross platform class path
  let mut class_path = String::from("libraries/*:fabric-libraries/*");
  if std::env::consts::OS == "windows" {
    class_path = class_path.replace(":", ";");
  }

  println!("launching {}", &instance);

  process::Command::new(&config.java.path)
    .current_dir(&path)
    .arg("-Dminecraft.launcher.brand=runmc")
    .arg(format!("-Xmx{}", &config.java.memory))
    .arg(format!("-Xms{}", &config.java.memory))
    .arg("-cp")
    .arg(&class_path)
    .arg(&instance_info.main_class)
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
    .arg(&instance)
    .stdout(process::Stdio::inherit())
    .stderr(process::Stdio::inherit())
    .spawn()
    .map_err(|e| e.to_string())?;

  Ok(())
}
