use crate::util::get_base_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::command;

#[derive(Serialize, Deserialize)]
pub struct Account {
  pub name: String,
  pub id: String,
  pub access_token: String,
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