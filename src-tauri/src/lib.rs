mod cryptography;
mod database;
mod invokable;
mod validator;

use database::Database;
use dotenv::dotenv;
use invokable::auth::{AuthProcedure, AuthResolver};
use tauri::async_runtime;
use taurpc::Router;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenv().ok();

    let router = Router::new().merge(AuthResolver.into_handler());

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
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
