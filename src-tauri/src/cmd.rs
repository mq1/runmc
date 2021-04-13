use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  MyCustomCommand { argument: String },
  DownloadFile { url: String, path: String, callback: String, error: String },
  StartMinecraft { version: String, access_token: String, callback: String, error: String }
}
