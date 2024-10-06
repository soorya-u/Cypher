mod cryptography;
mod database;
mod invokable;
mod validator;

use database::initialize_database;
use invokable::{login, sign_up};
use tauri::async_runtime::spawn;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|webview, _| {
            let w = webview.clone();
            spawn(async move {
                match initialize_database().await {
                    Ok(_) => {}
                    Err(_) => w.close().unwrap(),
                };
            });
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, login, sign_up])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
