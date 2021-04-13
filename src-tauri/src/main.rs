#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            },
            DownloadFile { url, path, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let base_path = tauri::api::path::home_dir().unwrap();
                let full_path = base_path.join(path);

                // create parent dirs
                let parent_dir = full_path.parent().unwrap();
                std::fs::create_dir_all(parent_dir).unwrap();

                // download only if file doesn't already exist
                if !full_path.exists() {
                  println!("downloading {} to {:?}", url, full_path);
                  let mut resp = reqwest::blocking::get(url).expect("File download failed");
                  let mut out = std::fs::File::create(full_path).expect("File creation failed");
                  std::io::copy(&mut resp, &mut out).expect("File writing failed");
                  println!("file downloaded");
                } else {
                  println!("{:?} already present", full_path);
                }

                Ok(())
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
