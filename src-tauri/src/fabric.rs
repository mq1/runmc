use crate::util::download_file;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tauri::command;

#[derive(Deserialize, Serialize, Clone)]
pub struct Loader {
  version: String,
  stable: bool,
}

#[command]
pub async fn get_fabric_loader_versions(game_version: String) -> Result<Vec<Loader>, String> {
  #[derive(Deserialize)]
  struct LoaderVersion {
    loader: Loader,
  }

  let res = reqwest::get(format!(
    "https://meta.fabricmc.net/v2/versions/loader/{}/",
    &game_version
  ))
  .await
  .map_err(|e| e.to_string())?;
  let j: Vec<LoaderVersion> = res.json().await.map_err(|e| e.to_string())?;

  let loaders = j.iter().map(|lv| lv.loader.clone()).collect();

  Ok(loaders)
}

#[command]
pub async fn install_fabric(game_version: String, loader_version: String) -> Result<(), String> {
  let path = Path::new("versions").join(&game_version);

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
    let path = path
      .join("fabric-libraries")
      .join(format!("{}-{}.jar", vec[1], vec[2]));
    download_file(url, path).await?;
  }

  // save main class name
  fs::write(
    path.join("info.json"),
    format!("{{ mainClass: \"{}\" }}", j.launcher_meta.main_class.client),
  )
  .map_err(|e| e.to_string())?;

  Ok(())
}
