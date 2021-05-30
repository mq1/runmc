use crate::{util::get_base_dir, config::get_config};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::command;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  pub name: String,
  pub id: String,
  pub access_token: String,
}

#[command]
pub fn get_accounts() -> Result<Vec<Account>, String> {
  let path = get_base_dir()?.join("accounts.yaml");

  if !&path.exists() {
    return Ok(vec![]);
  }

  let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
  let accounts: Vec<Account> = serde_yaml::from_str(&text).map_err(|e| e.to_string())?;

  Ok(accounts)
}

#[command]
pub fn save_accounts(accounts: Vec<Account>) -> Result<(), String> {
  let path = get_base_dir()?.join("accounts.yaml");

  let text = serde_yaml::to_string(&accounts).map_err(|e| e.to_string())?;
  fs::write(&path, &text).map_err(|e| e.to_string())?;

  Ok(())
}

#[command]
pub async fn login(email: String, password: String) -> Result<(), String> {
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
  #[serde(rename_all = "camelCase")]
  struct Payload {
    username: String,
    password: String,
    agent: Agent,
    client_token: String,
  }

  let config = get_config()?;

  let payload = Payload {
    username: email,
    password: password,
    agent: Agent {
      name: String::from("Minecraft"),
      version: 1,
    },
    client_token: config.client_id,
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

  add_account(account)?;
  println!("account added");

  Ok(())
}

pub fn add_account(account: Account) -> Result<(), String> {
  let mut accounts = get_accounts()?;
  accounts.push(account);
  save_accounts(accounts)?;

  Ok(())
}

#[command]
pub fn remove_account(account: Account) -> Result<(), String> {
  // parse accounts
  let mut accounts = get_accounts()?;

  // remove the account
  accounts.retain(|a| a.id != account.id);

  // save accounts
  save_accounts(accounts)?;

  Ok(())
}

pub async fn refresh_account(account: Account) -> Result<Account, String> {
  let config = get_config()?;

  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
  struct Payload {
    access_token: String,
    client_token: String,
  }

  let payload = Payload {
    access_token: String::from(&account.access_token),
    client_token: config.client_id,
  };

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Json {
    access_token: String,
  }

  let client = reqwest::Client::new();
  let res = client
    .post("https://authserver.mojang.com/refresh")
    .json(&payload)
    .send()
    .await
    .map_err(|e| e.to_string())?;
  let j: Json = res.json().await.map_err(|e| e.to_string())?;

  let mut account = account;
  account.access_token = j.access_token;

  // update accounts
  remove_account(account.clone())?;
  add_account(account.clone())?;

  Ok(account)
}
