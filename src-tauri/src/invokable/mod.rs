use std::error::Error;

pub mod auth;

pub enum ErrorType {
    None = 0,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ErrorPayload {
    pub message: String,
    pub error: String,
}

impl ErrorPayload {
    pub fn from_message_with_closure(m: &str) -> impl FnOnce(String) -> Self {
        let message = String::from(m);
        |error| ErrorPayload { error, message }
    }
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
