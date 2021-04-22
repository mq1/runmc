use crate::instance::instance_info;
use crate::util::download_file;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;

#[derive(Deserialize, Serialize, Clone)]
pub struct Loader {
  version: String,
  stable: bool,
  maven: String,
}

#[command]
pub async fn get_fabric_loader_versions(instance_name: String) -> Result<Vec<Loader>, String> {
  #[derive(Deserialize)]
  struct LoaderVersion {
    loader: Loader,
  }

  let info = instance_info(instance_name)?;

  let res = reqwest::get(format!(
    "https://meta.fabricmc.net/v2/versions/loader/{}/",
    &info.game_version
  ))
  .await
  .map_err(|e| e.to_string())?;
  let j: Vec<LoaderVersion> = res.json().await.map_err(|e| e.to_string())?;

  let loaders = j.iter().map(|lv| lv.loader.clone()).collect();

  Ok(loaders)
}

pub async fn download_fabric(
  dir: &PathBuf,
  game_version: &String,
  loader_version: &String,
) -> Result<String, String> {
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
    launcher_meta: LauncherMeta,
  }

  let res = reqwest::get(format!(
    "https://meta.fabricmc.net/v2/versions/loader/{}/{}/",
    &game_version, &loader_version
  ))
  .await
  .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;

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
    let path = dir
      .join("libraries") // TODO separate fabric-libraries from libraries
      .join(format!("{}-{}.jar", vec[1], vec[2]));
    download_file(url, path).await?;
  }

  // download loader jar
  let split = j.loader.maven.split(":");
  let vec = split.collect::<Vec<&str>>();
  let url = format!("https://maven.fabricmc.net/{}/{}/{}/{}-{}.jar", vec[0].replace(".", "/"), vec[1], vec[2], vec[1], vec[2]);
  let path = dir.join("libraries").join(format!("{}-{}.jar", vec[1], vec[2]));
  download_file(url, path).await?;

  Ok(j.launcher_meta.main_class.client)
}
