use crate::account;
use crate::config;
use crate::util;
use crate::version;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf, process};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
  pub game_version: String,
  pub main_class: String,
  pub fabric: bool,
}

impl AsRef<InstanceInfo> for InstanceInfo {
  fn as_ref(&self) -> &InstanceInfo {
    self
  }
}

pub fn get_path<S: AsRef<str>>(instance_name: S) -> Result<PathBuf, Box<dyn Error>> {
  let path = util::get_base_dir()?
    .join("instances")
    .join(instance_name.as_ref());

  Ok(path)
}

pub fn read_info<S: AsRef<str>>(instance_name: S) -> Result<InstanceInfo, Box<dyn Error>> {
  let path = get_path(instance_name)?.join("info.yaml");
  let info = fs::read_to_string(path)?;
  let info: InstanceInfo = serde_yaml::from_str(&info)?;

  Ok(info)
}

pub fn save_info<S: AsRef<str>, II: AsRef<InstanceInfo>>(
  instance_name: S,
  info: II,
) -> Result<(), Box<dyn Error>> {
  let path = get_path(instance_name)?.join("info.yaml");
  let text = serde_yaml::to_string(info.as_ref())?;
  fs::write(path, text)?;

  Ok(())
}

pub fn list() -> Result<Vec<String>, Box<dyn Error>> {
  let path = util::get_base_dir()?.join("instances");
  let entries = util::ls(path)?;

  Ok(entries)
}

pub async fn new<S: AsRef<str>, MV: AsRef<version::MinecraftVersion>>(
  name: S,
  game_version: MV,
) -> Result<(), Box<dyn Error>> {
  let path = get_path(name.as_ref())?;
  let main_class = version::download_version(path, game_version.as_ref()).await?;

  let instance_info = InstanceInfo {
    game_version: game_version.as_ref().id.clone(),
    main_class: main_class,
    fabric: false,
  };

  save_info(name.as_ref(), instance_info)?;

  println!("instance created");

  Ok(())
}

pub fn rename<S: AsRef<str>>(current_name: S, new_name: S) -> Result<(), Box<dyn Error>> {
  let instances_dir = util::get_base_dir()?.join("instances");

  fs::rename(
    instances_dir.join(current_name.as_ref()),
    instances_dir.join(new_name.as_ref()),
  )?;

  Ok(())
}

pub fn delete<S: AsRef<str>>(name: S) -> Result<(), Box<dyn Error>> {
  let path = get_path(name.as_ref())?;
  fs::remove_dir_all(&path)?;

  println!("deleted {:?}", path);
  Ok(())
}

pub fn list_mods<S: AsRef<str>>(instance_name: S) -> Result<Vec<String>, Box<dyn Error>> {
  let path = get_path(instance_name.as_ref())?.join("mods");
  let entries = util::ls(path)?;

  Ok(entries)
}

pub fn open_dir<S: AsRef<str>>(instance_name: S) -> Result<(), Box<dyn Error>> {
  let path = get_path(instance_name.as_ref())?;
  let path = format!("{:?}", path);

  println!("Opening {}",  &path);
  tauri::api::shell::open(path, None)?;

  Ok(())
}

pub fn open_mods_dir<S: AsRef<str>>(instance_name: S) -> Result<(), Box<dyn Error>> {
  let path = get_path(instance_name.as_ref())?.join("mods");
  let path = format!("{:?}", path);
  tauri::api::shell::open(path, None)?;

  Ok(())
}

pub async fn run<S: AsRef<str>, A: AsRef<account::Account>>(
  instance_name: S,
  account: A,
) -> Result<(), Box<dyn Error>> {
  let path = get_path(&instance_name)?;
  let config = config::read()?;

  let instance_info = read_info(&instance_name)?;

  // cross platform class path
  let mut class_path = String::from("libraries/*:fabric-libraries/*");
  if std::env::consts::OS == "windows" {
    class_path = class_path.replace(":", ";");
  }

  // refresh access token
  println!("refreshing access token");
  let account = account::refresh(account.as_ref()).await?;

  println!("launching {}", instance_name.as_ref());

  process::Command::new(config.java.path)
    .current_dir(path)
    .arg("-Dminecraft.launcher.brand=runmc")
    .arg(format!("-Xmx{}", config.java.memory))
    .arg(format!("-Xms{}", config.java.memory))
    .arg(format!("-Dorg.lwjgl.librarypath=/Applications/MultiMC.app/Contents/MacOS/lwjglnatives/"))
    .arg(format!("-Dfml.earlyprogresswindow=false"))
    .arg("-cp")
    .arg(class_path)
    .arg(instance_info.main_class)
    .arg("--gameDir")
    .arg(".")
    .arg("--assetsDir")
    .arg("../../assets")
    .arg("--assetIndex")
    .arg(format!("../../instances/{}/asset-index", instance_name.as_ref()))
    .arg("--username")
    .arg(&account.as_ref().name)
    .arg("--uuid")
    .arg(&account.as_ref().id)
    .arg("--accessToken")
    .arg(&account.as_ref().access_token)
    .arg("--userType")
    .arg("mojang")
    .arg("--version")
    .arg(instance_name.as_ref())
    .stdout(process::Stdio::inherit())
    .stderr(process::Stdio::inherit())
    .spawn()?;

  Ok(())
}
