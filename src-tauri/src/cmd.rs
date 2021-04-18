use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::{
  collections::HashMap,
  fs, io,
  path::{Path, PathBuf},
  process,
};
use tauri::command;

fn get_base_dir() -> Result<PathBuf, String> {
  let base_path = tauri::api::path::home_dir()
    .ok_or("Cannot find base home dir")?
    .join(".runmc");

  Ok(base_path)
}

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

async fn download_file(url: String, path: PathBuf) -> Result<(), String> {
  let path = get_base_dir()?.join(path);

  // create parent dirs
  let parent_dir = path.parent().ok_or("cannot retrieve parent dir")?;
  fs::create_dir_all(parent_dir).map_err(|e| e.to_string())?;

  // download only if file doesn't already exist
  if !path.exists() {
    println!("downloading {} to {:?}", url, path);
    let resp = reqwest::get(url)
      .await
      .map_err(|e| e.to_string())?
      .bytes()
      .await
      .map_err(|e| e.to_string())?;
    let mut bytes = resp.as_ref();

    let mut out = fs::File::create(path).map_err(|e| e.to_string())?;
    io::copy(&mut bytes, &mut out).map_err(|e| e.to_string())?;
    println!("file downloaded");
  } else {
    println!("{:?} already present", path);
  }

  Ok(())
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

  println!();
  println!("version {} installed", &version_id);

  Ok(())
}

#[command]
pub fn remove_version(version: String) -> Result<(), String> {
  let path = get_base_dir()?.join("versions").join(&version);
  fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
  println!("deleted {:?}", &path);

  Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Account {
  name: String,
  id: String,
  access_token: String,
}

#[command]
pub fn run_minecraft(version: String, account: Account) -> Result<(), String> {
  let path = get_base_dir()?.join("versions").join(&version);

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

  process::Command::new("java")
    .current_dir(path)
    .arg("-Dminecraft.launcher.brand=runmc")
    .arg("-Xmx2G")
    .arg("-Xms2G")
    .arg("-cp")
    .arg("libraries/*")
    .arg("net.minecraft.client.main.Main")
    .arg("--gameDir")
    .arg("game-data")
    .arg("--assetsDir")
    .arg("assets")
    .arg("--assetIndex")
    .arg(asset_index)
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

#[command]
async fn login(email: String, password: String) -> Result<(), String> {
  println!("trying to add account {}", &email);

  #[derive(Deserialize)]
  struct Profile {
    name: String,
    id: String,
  }

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Json {
    access_token: String,
    selected_profile: Profile,
  }

  #[derive(Serialize)]
  struct Agent {
    name: String,
    version: i32,
  }

  #[derive(Serialize)]
  struct Payload {
    username: String,
    password: String,
    agent: Agent,
  }

  let payload = Payload {
    username: email,
    password: password,
    agent: Agent {
      name: String::from("Minecraft"),
      version: 1,
    },
  };

  let client = reqwest::Client::new();
  let res = client
    .post("https://authserver.mojang.com/authenticate")
    .json(&payload)
    .send()
    .await
    .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;

  let account = Account {
    name: j.selected_profile.name,
    id: j.selected_profile.id,
    access_token: j.access_token,
  };

  let mut accounts = vec![account];

  // save login to file
  let path = get_base_dir()?.join("accounts.json");
  if (&path).exists() {
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut existing_accounts: Vec<Account> =
      serde_json::from_str(&text).map_err(|e| e.to_string())?;
    accounts.append(&mut existing_accounts);
  }

  let json_string = serde_json::to_string(&accounts).map_err(|e| e.to_string())?;
  fs::write(&path, json_string).map_err(|e| e.to_string())?;

  println!("account added");

  Ok(())
}

#[command]
pub fn accounts() -> Result<Vec<Account>, String> {
  let path = get_base_dir()?.join("accounts.json");

  let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
  let accounts: Vec<Account> = serde_json::from_str(&text).map_err(|e| e.to_string())?;

  Ok(accounts)
}

#[command]
pub fn remove_account(name: String) -> Result<(), String> {
  let path = get_base_dir()?.join("accounts.json");

  // parse accounts
  let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
  let mut accounts: Vec<Account> = serde_json::from_str(&text).map_err(|e| e.to_string())?;

  // remove the account
  accounts.retain(|a| a.name != name);

  // save accounts
  let json_string = serde_json::to_string(&accounts).map_err(|e| e.to_string())?;
  fs::write(&path, json_string).map_err(|e| e.to_string())?;

  Ok(())
}
