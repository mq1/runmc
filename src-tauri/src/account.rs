use crate::{config, util};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  pub name: String,
  pub id: String,
  pub access_token: String,
}

impl AsRef<Account> for Account {
  fn as_ref(&self) -> &Account {
    self
  }
}

pub fn list() -> Result<Vec<Account>, Box<dyn Error>> {
  let path = util::get_base_dir()?.join("accounts.yaml");

  if !&path.exists() {
    return Ok(vec![]);
  }

  let text = fs::read_to_string(&path)?;
  let accounts: Vec<Account> = serde_yaml::from_str(&text)?;

  Ok(accounts)
}

fn save<V: AsRef<Vec<Account>>>(accounts: V) -> Result<(), Box<dyn Error>> {
  let path = util::get_base_dir()?.join("accounts.yaml");

  let text = serde_yaml::to_string(accounts.as_ref())?;
  fs::write(&path, &text)?;

  Ok(())
}

pub async fn login<S: AsRef<str>>(email: S, password: S) -> Result<(), Box<dyn Error>> {
  println!("trying to add account {}", email.as_ref());

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

  let config = config::read()?;

  let payload = Payload {
    username: email.as_ref().to_string(),
    password: password.as_ref().to_string(),
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
    .await?;
  let j: Json = res.json().await?;

  let account = Account {
    name: j.selected_profile.name,
    id: j.selected_profile.id,
    access_token: j.access_token,
  };

  add(&account)?;
  println!("account added");

  Ok(())
}

pub fn add<A: AsRef<Account>>(account: A) -> Result<(), Box<dyn Error>> {
  let mut accounts = list()?;
  let account = account.as_ref().clone();
  accounts.push(account);
  save(&accounts)?;

  Ok(())
}

pub fn remove<A: AsRef<Account>>(account: A) -> Result<(), Box<dyn Error>> {
  // parse accounts
  let mut accounts = list()?;

  // remove the account
  accounts.retain(|a| a.id != account.as_ref().id);

  // save accounts
  save(&accounts)?;

  Ok(())
}

pub async fn refresh<A: AsRef<Account>>(account: A) -> Result<Account, Box<dyn Error>> {
  let config = config::read()?;

  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
  struct Payload {
    access_token: String,
    client_token: String,
  }

  let payload = Payload {
    access_token: account.as_ref().access_token.clone(),
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
    .await?;
  let j: Json = res.json().await?;

  let mut account = account.as_ref().clone();
  account.access_token = j.access_token;

  // update accounts
  remove(&account)?;
  add(&account)?;

  Ok(account)
}
