use crate::util::download_file;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
};
use tauri::command;

#[derive(Serialize, Deserialize)]
pub struct GameVersion {
  pub id: String,
  r#type: String,
  url: String,
}

#[command]
pub async fn list_available_game_versions() -> Result<Vec<GameVersion>, String> {
  #[derive(Deserialize)]
  struct Json {
    versions: Vec<GameVersion>,
  }

  let res = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
    .await
    .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;

  Ok(j.versions)
}

#[derive(Deserialize)]
struct Artifact {
  path: String,
  url: String,
}

#[derive(Deserialize)]
struct LibraryDownloads {
  artifact: Artifact,
  classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(Deserialize)]
struct Library {
  downloads: LibraryDownloads,
  natives: Option<HashMap<String, String>>,
  rules: Option<Vec<Value>>,
}

async fn download_libraries(dir: PathBuf, libraries: Vec<Library>) -> Result<(), String> {
  // find platform (os)
  let platform = std::env::consts::OS;
  let platform = platform.replace("macos", "osx");

  for library in libraries {
    if (&library.rules).is_some() {
      // TODO FIX this dumpster fire of code
      let rules = library.rules.unwrap();

      if rules.len() == 1 && platform != "osx" {
        continue;
      }
      if rules.len() == 2 && platform == "osx" {
        continue;
      }
    }

    let file_path = Path::new(&library.downloads.artifact.path);
    let file_name = file_path.file_name().ok_or(format!(
      "failed to get {} file name",
      library.downloads.artifact.path
    ))?;
    download_file(library.downloads.artifact.url, dir.join(file_name)).await?;

    if library.natives.is_some() {
      let natives = library.natives.unwrap();
      if natives.contains_key(&platform) {
        let classifiers = library.downloads.classifiers.unwrap();
        let artifact = &classifiers[&natives[&platform]];
        let file_name = Path::new(&artifact.path).file_name().ok_or(format!(
          "failed to get {} file name",
          library.downloads.artifact.path
        ))?;

        download_file(String::from(&artifact.url), dir.join(file_name)).await?;
      }
    }
  }

  Ok(())
}

#[derive(Deserialize)]
struct AssetIndex {
  id: String,
  url: String,
}

#[derive(Deserialize)]
struct Object {
  hash: String,
}

async fn download_objects(dir: PathBuf, objects: Vec<&Object>) -> Result<(), String> {
  for object in objects {
    let path = format!("{}/{}", &object.hash[..2], &object.hash);
    let url = format!("https://resources.download.minecraft.net/{}", &path);

    download_file(url, dir.join(path)).await?;
  }

  Ok(())
}

async fn download_assets(dir: PathBuf, asset_index: AssetIndex) -> Result<(), String> {
  #[derive(Deserialize)]
  struct Json {
    objects: HashMap<String, Object>,
  }

  let res = reqwest::get(&asset_index.url)
    .await
    .map_err(|e| e.to_string())?;
  let text = res.text().await.map_err(|e| e.to_string())?;
  let j: Json = serde_json::from_str(&text).map_err(|e| e.to_string())?;

  download_objects(dir.join("objects"), j.objects.values().collect()).await?;

  // save asset index json
  let asset_index_file_name = format!("{}.json", &asset_index.id);
  let dir = dir.join("indexes");
  fs::create_dir(&dir).map_err(|e| e.to_string())?;
  fs::write(&dir.join(asset_index_file_name), &text).map_err(|e| e.to_string())?;

  Ok(())
}

#[derive(Deserialize)]
struct ObjectContainingURL {
  url: String,
}

async fn download_client(dir: PathBuf, client: ObjectContainingURL) -> Result<(), String> {
  let path = dir.join("client.jar");
  download_file(client.url, path).await?;

  Ok(())
}

pub async fn download_version(dir: &PathBuf, game_version: &GameVersion) -> Result<String, String> {
  #[derive(Deserialize)]
  struct Downloads {
    client: ObjectContainingURL,
  }

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Json {
    downloads: Downloads,
    libraries: Vec<Library>,
    asset_index: AssetIndex,
    main_class: String,
  }

  let res = reqwest::get(&game_version.url)
    .await
    .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;
  let version_id = String::from(&game_version.id);

  download_client(dir.join("libraries"), j.downloads.client).await?;
  download_libraries(dir.join("libraries"), j.libraries).await?;
  download_assets(dir.join("assets"), j.asset_index).await?;

  println!("\nversion {} installed", &version_id);
  Ok(j.main_class)
}
