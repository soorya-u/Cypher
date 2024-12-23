use std::error::Error;

pub mod auth;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ErrorAction {
    None,
    Redirect,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ErrorType {
    Expected,
    User,
    Internal,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ErrorPayload {
    pub error_type: ErrorType,
    pub message: String,
    pub error: String,
    pub details: String,
    pub action_type: ErrorAction,
}

impl ErrorPayload {
    pub fn from_message_with_closure(
        error_type: ErrorType,
        m: &str,
        action_type: ErrorAction,
    ) -> impl FnOnce(String) -> Self {
        let message = m.to_string();
        |error| ErrorPayload {
            details: error.clone(),
            error,
            message,
            action_type,
            error_type,
        }
    }

    pub fn from_error_with_closure<E>(
        error_type: ErrorType,
        m: &str,
        action_type: ErrorAction,
        dev: &str,
    ) -> impl FnOnce(E) -> Self
    where
        E: Error,
    {
        let message = m.to_string();
        let details = dev.to_string();
        |err| ErrorPayload {
            error: err.to_string(),
            message,
            error_type,
            action_type,
            details,
        }
    }
}
