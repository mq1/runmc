use crate::util::{download_file, get_base_dir};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::{collections::HashMap, fs, path::Path};
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
  let path = get_base_dir()?.join("versions");

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

async fn download_libraries(version: String, libraries: Vec<Library>) -> Result<(), String> {
  let path = Path::new("versions").join(version).join("libraries");

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
    download_file(library.downloads.artifact.url, path.join(file_name)).await?;

    if library.natives.is_some() {
      let natives = library.natives.unwrap();
      if natives.contains_key(&platform) {
        let classifiers = library.downloads.classifiers.unwrap();
        let artifact = &classifiers[&natives[&platform]];
        let file_name = Path::new(&artifact.path).file_name().ok_or(format!(
          "failed to get {} file name",
          library.downloads.artifact.path
        ))?;

        download_file(String::from(&artifact.url), path.join(file_name)).await?;
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

async fn download_assets(version: String, asset_index: AssetIndex) -> Result<(), String> {
  let path = Path::new("versions")
    .join(&version)
    .join("assets")
    .join("objects");

  #[derive(Deserialize)]
  struct Object {
    hash: String,
  }

  #[derive(Deserialize)]
  struct Json {
    objects: HashMap<String, Object>,
  }

  let res = reqwest::get(&asset_index.url)
    .await
    .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;

  for object in j.objects.values() {
    let dir = &object.hash[..2];
    let url = format!(
      "https://resources.download.minecraft.net/{}/{}",
      dir, &object.hash
    );
    let path = path.join(dir).join(&object.hash);

    download_file(url, path).await?;
  }

  // save asset index json
  let path = Path::new("versions")
    .join(&version)
    .join("assets")
    .join("indexes")
    .join(format!("{}.json", &asset_index.id));
  download_file(String::from(&asset_index.url), path).await?;

  Ok(())
}

#[command]
pub async fn install_version(version: Version) -> Result<(), String> {
  #[derive(Deserialize)]
  struct ObjectContainingURL {
    url: String,
  }

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

  let res = reqwest::get(version.url).await.map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;
  let version_id = String::from(version.id);

  download_file(
    j.downloads.client.url,
    Path::new("versions")
      .join(&version_id)
      .join("libraries")
      .join("client.jar"),
  )
  .await?;
  download_libraries(String::from(&version_id), j.libraries).await?;
  download_assets(String::from(&version_id), j.asset_index).await?;

  // save main class name
  let path = get_base_dir()?.join("versions").join(&version_id);
  fs::write(path.join("info.json"), format!("{{ \"mainClass\": \"{}\" }}", j.main_class)).map_err(|e| e.to_string())?;

  println!();
  println!("version {} installed", &version_id);

  Ok(())
}

#[command]
pub fn rename_version(version: String, name: String) -> Result<(), String> {
  let path = get_base_dir()?.join("versions").join(&version);
  let final_path = get_base_dir()?.join("versions").join(&name);

  fs::rename(&path, &final_path).map_err(|e| e.to_string())?;

  Ok(())
}

#[command]
pub fn remove_version(version: String) -> Result<(), String> {
  let path = get_base_dir()?.join("versions").join(&version);
  fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
  println!("deleted {:?}", &path);

  Ok(())
}
