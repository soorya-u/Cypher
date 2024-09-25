mod auth;
mod callable;
mod cryptography;
mod database;

use database::initialize_database;
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
                    Err(_) => w.close().unwrap(),
                    Ok(_) => println!("Init Successfull"),
                };
            });
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
