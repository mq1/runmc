use serde::{Deserialize, Serialize};

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
