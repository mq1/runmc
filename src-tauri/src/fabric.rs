use crate::{instance, util};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Deserialize, Serialize, Clone)]
pub struct Loader {
  version: String,
  stable: bool,
  maven: String,
}

impl AsRef<Loader> for Loader {
  fn as_ref(&self) -> &Loader {
    self
  }
}

pub async fn get_loader_versions<S: AsRef<str>>(
  mc_version: S,
) -> Result<Vec<Loader>, Box<dyn Error>> {
  #[derive(Deserialize)]
  struct LoaderVersion {
    loader: Loader,
  }

  let res = reqwest::get(format!(
    "https://meta.fabricmc.net/v2/versions/loader/{}/",
    mc_version.as_ref()
  ))
  .await?;

  let j = res.json::<Vec<LoaderVersion>>().await?;

  let loaders = j.into_iter().map(|lv| lv.loader).collect();

  Ok(loaders)
}

fn maven_download_url<S: AsRef<str>>(maven: S) -> (String, String) {
  let path = maven
    .as_ref()
    .split(":")
    .map(|a| a.to_string())
    .collect::<Vec<String>>();

  let url = format!(
    "https://maven.fabricmc.net/{}/{}/{}/{}-{}.jar",
    path[0].replace(".", "/"),
    path[1],
    path[2],
    path[1],
    path[2]
  );

  let file_name = format!("{}-{}.jar", path[1], path[2]);

  (url, file_name)
}

pub async fn install<S: AsRef<str>>(
  instance_name: S,
  loader_version: S,
) -> Result<(), Box<dyn Error>> {
  #[derive(Deserialize)]
  struct CommonUnit {
    name: String,
    url: String,
  }

  #[derive(Deserialize)]
  struct Libraries {
    common: Vec<CommonUnit>,
  }

  #[derive(Deserialize)]
  struct MainClass {
    client: String,
  }

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct LauncherMeta {
    libraries: Libraries,
    main_class: MainClass,
  }

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Json {
    loader: Loader,
    intermediary: Loader,
    launcher_meta: LauncherMeta,
  }

  let path = instance::get_path(&instance_name)?.join("fabric-libraries");
  let info = instance::read_info(&instance_name)?;

  let res = reqwest::get(format!(
    "https://meta.fabricmc.net/v2/versions/loader/{}/{}/",
    info.game_version,
    loader_version.as_ref()
  ))
  .await?;
  let j: Json = res.json().await?;

  for item in j.launcher_meta.libraries.common {
    let split = item.name.split(":");
    let vec = split.collect::<Vec<&str>>();

    let url = format!(
      "{}{}/{}/{}/{}-{}.jar",
      item.url,
      vec[0].replace(".", "/"),
      vec[1],
      vec[2],
      vec[1],
      vec[2]
    );
    let path = path.join(format!("{}-{}.jar", vec[1], vec[2]));
    util::download_file(&url, &path).await?;
  }

  // download loader jar
  let (url, file_name) = maven_download_url(&j.loader.maven);
  let path = path.join(&file_name);
  util::download_file(&url, &path).await?;

  // download intermediary jar
  let (url, file_name) = maven_download_url(&j.intermediary.maven);
  let path = path.join(&file_name);
  util::download_file(url, &path).await?;

  // write new instance info
  let mut info = info;
  info.main_class = j.launcher_meta.main_class.client;
  info.fabric = true;
  instance::save_info(instance_name, info)?;

  // create mods dir
  let path = path.join("mods");
  fs::create_dir_all(path)?;

  println!("\nfabric installed");

  Ok(())
}
