mod cryptography;
mod database;
mod invokable;
mod validator;

use database::Database;
use dotenv::dotenv;
use invokable::auth::{get_session, login, logout, sign_up};
use tauri::async_runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

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
        .invoke_handler(tauri::generate_handler![
            login,
            sign_up,
            get_session,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
