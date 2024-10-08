mod cryptography;
mod database;
mod invokable;
mod validator;

use database::Database;
use invokable::{greet, login, sign_up};
use tauri::async_runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|webview, _| {
            let w = webview.clone();
            async_runtime::spawn(async move {
                match Database::new().await {
                    Ok(_) => {}
                    Err(_) => w.close().unwrap(),
                }
            });
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, login, sign_up])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
