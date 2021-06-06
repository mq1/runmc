use crate::util;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaConfig {
  pub path: String,
  pub memory: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub client_id: String,
  pub java: JavaConfig,
  pub locale: String,
}

impl AsRef<Config> for Config {
  fn as_ref(&self) -> &Config {
    self
  }
}

pub fn get_defaults() -> Config {
  Config {
    client_id: Uuid::new_v4().to_string(),
    java: JavaConfig {
      path: String::from("javaw"),
      memory: String::from("2G"),
    },
    locale: String::from("en"),
  }
}

pub fn read() -> Result<Config, Box<dyn Error>> {
  let path = util::get_base_dir()?.join("config.yaml");
  let config: Config;

  if !path.exists() {
    // write the default config if not found
    config = get_defaults();
    let text = serde_yaml::to_string(&config)?;
    fs::write(&path, &text)?;
  } else {
    let text = fs::read_to_string(&path)?;
    config = serde_yaml::from_str(&text)?;
  }

  Ok(config)
}

pub fn save<C: AsRef<Config>>(config: C) -> Result<(), Box<dyn Error>> {
  let path = util::get_base_dir()?.join("config.yaml");
  let text = serde_yaml::to_string(config.as_ref())?;
  fs::write(&path, &text)?;

  Ok(())
}
