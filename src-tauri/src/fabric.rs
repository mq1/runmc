use crate::instance::get_instance_info;
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

  let info = get_instance_info(String::from(&instance_name))?;

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

fn maven_download_url(maven: &String) -> (String, String) {
  let split = maven.split(":");

  let vec = split.collect::<Vec<&str>>();

  let url = format!(
    "https://maven.fabricmc.net/{}/{}/{}/{}-{}.jar",
    vec[0].replace(".", "/"),
    vec[1],
    vec[2],
    vec[1],
    vec[2]
  );

  let file_name = format!("{}-{}.jar", vec[1], vec[2]);

  (url, file_name)
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
    intermediary: Loader,
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
      .join("fabric-libraries")
      .join(format!("{}-{}.jar", vec[1], vec[2]));
    download_file(url, path).await?;
  }

  // download loader jar
  let (url, file_name) = maven_download_url(&j.loader.maven);
  let path = dir.join("fabric-libraries").join(&file_name);
  download_file(url, path).await?;

  // download intermediary jar
  let (url, file_name) = maven_download_url(&j.intermediary.maven);
  let path = dir.join("fabric-libraries").join(&file_name);
  download_file(url, path).await?;

  Ok(j.launcher_meta.main_class.client)
}
