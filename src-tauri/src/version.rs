use crate::util;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::{collections::HashMap, error::Error, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct MinecraftVersion {
  pub id: String,
  r#type: String,
  url: String,
}

impl AsRef<MinecraftVersion> for MinecraftVersion {
  fn as_ref(&self) -> &MinecraftVersion {
    self
  }
}

#[derive(Deserialize)]
struct ObjectContainingURL {
  url: String,
}

impl AsRef<ObjectContainingURL> for ObjectContainingURL {
  fn as_ref(&self) -> &ObjectContainingURL {
    self
  }
}

pub async fn list_available() -> Result<Vec<MinecraftVersion>, Box<dyn Error>> {
  #[derive(Deserialize)]
  struct Json {
    versions: Vec<MinecraftVersion>,
  }

  let res = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json").await?;
  let j: Json = res.json().await?;

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

async fn download_libraries<P: AsRef<Path>, LV: AsRef<Vec<Library>>>(
  dir: P,
  libraries: LV,
) -> Result<(), Box<dyn Error>> {
  // find platform (os)
  let platform = std::env::consts::OS;
  let platform = platform.replace("macos", "osx");

  for library in libraries.as_ref() {
    if (library.rules).is_some() {
      // TODO FIX this dumpster fire of code
      let rules = library.rules.as_ref().unwrap();

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
    util::download_file(
      &library.downloads.artifact.url,
      dir.as_ref().join(file_name),
    )
    .await?;

    if library.natives.is_some() {
      let natives = library.natives.as_ref().unwrap();
      if natives.contains_key(&platform) {
        let classifiers = library.downloads.classifiers.as_ref().unwrap();
        let artifact = &classifiers[&natives[&platform]];
        let file_name = Path::new(&artifact.path).file_name().ok_or(format!(
          "failed to get {} file name",
          library.downloads.artifact.path
        ))?;

        util::download_file(artifact.url.clone(), dir.as_ref().join(file_name)).await?;
      }
    }
  }

  Ok(())
}

#[derive(Deserialize, Clone)]
struct Object {
  hash: String,
}

async fn download_objects<P: AsRef<Path>, OV: AsRef<Vec<Object>>>(
  dir: P,
  objects: OV,
) -> Result<(), Box<dyn Error>> {
  for object in objects.as_ref() {
    let path = format!("{}/{}", &object.hash[..2], &object.hash);
    let url = format!("https://resources.download.minecraft.net/{}", &path);

    util::download_file(url, dir.as_ref().join(path)).await?;
  }

  Ok(())
}

async fn download_assets<P: AsRef<Path>, O: AsRef<ObjectContainingURL>>(
  instance_dir: P,
  asset_index: O,
) -> Result<(), Box<dyn Error>> {
  #[derive(Deserialize)]
  struct Json {
    objects: HashMap<String, Object>,
  }

  let res = reqwest::get(&asset_index.as_ref().url).await?;
  let text = res.text().await?;
  let j: Json = serde_json::from_str(&text)?;

  let objects = j.objects.values().cloned().collect::<Vec<Object>>();
  let objects_dir = util::get_base_dir()?.join("assets").join("objects");
  download_objects(objects_dir, objects).await?;

  // save asset index json
  let path = instance_dir.as_ref().join("asset-index.json");
  fs::write(path, text)?;

  Ok(())
}

async fn download_client<P: AsRef<Path>, O: AsRef<ObjectContainingURL>>(
  dir: P,
  client: O,
) -> Result<(), Box<dyn Error>> {
  let path = dir.as_ref().join("client.jar");
  util::download_file(&client.as_ref().url, path).await?;

  Ok(())
}

pub async fn download_version<P: AsRef<Path>, GV: AsRef<MinecraftVersion>>(
  dir: P,
  game_version: GV,
) -> Result<String, Box<dyn Error>> {
  #[derive(Deserialize)]
  struct Downloads {
    client: ObjectContainingURL,
  }

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Json {
    downloads: Downloads,
    libraries: Vec<Library>,
    asset_index: ObjectContainingURL,
    main_class: String,
  }

  let res = reqwest::get(&game_version.as_ref().url).await?;
  let j: Json = res.json().await?;

  download_client(dir.as_ref().join("libraries"), j.downloads.client).await?;
  download_libraries(dir.as_ref().join("libraries"), j.libraries).await?;
  download_assets(dir, j.asset_index).await?;

  println!("\nversion {} installed", game_version.as_ref().id);

  Ok(j.main_class)
}
