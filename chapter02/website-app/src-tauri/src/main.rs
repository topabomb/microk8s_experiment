#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{api::process::{Command,CommandEvent},Manager};
fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      tauri::async_runtime::spawn(async move {
        let (mut rx, mut child) = Command::new_sidecar("backend-app")
          .expect("failed to setup `backend-app` sidecar")
          .spawn()
          .expect("Failed to spawn packaged node");
          tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
              if let CommandEvent::Stdout(line) = event {
                window
                  .emit("message", Some(format!("'{}'", line)))
                  .expect("failed to emit event");
                // write to stdin
                child.write("message from Rust\n".as_bytes()).unwrap();
              }
            }
          });
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");


}
