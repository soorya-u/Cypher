mod auth;
mod cryptography;
mod database;
mod callable;

use database::db_init;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|_, _| {
            println!("page loaded nicely");
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, db_init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
