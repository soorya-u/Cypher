use directories::ProjectDirs;
use kv::{Config, Store};
use std::fs::{self, File};

use crate::invokable::{ErrorAction, ErrorPayload, ErrorType};

// TODO: store should be bucket. I'll Make it Bucket when I understand lifetimes! :(
pub struct Vault {
    store: Store,
}

impl Vault {
    pub fn new() -> Result<Vault, ErrorPayload> {
        let secrets_file = String::from("session.secrets");

        let proj_dirs = ProjectDirs::from("dev", "soorya-u", "cypher")
            .ok_or_else(|| String::from("Failed to get project directories"))
            .map_err(ErrorPayload::from_message_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
            ))?;

        let secrets_path = proj_dirs.data_local_dir().join(&secrets_file);
        let parent_dir = secrets_path
            .parent()
            .ok_or_else(|| String::from("Failed to get parent directory"))
            .map_err(ErrorPayload::from_message_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
            ))?;
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Failed to create database directory",
            ))?;
        }

        if !parent_dir.exists() {
            File::create(secrets_path.clone()).map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to create secrets file",
            ))?;
        }

        let cnf = Config::new(secrets_path);
        let store = Store::new(cnf).map_err(ErrorPayload::from_error_with_closure(
            ErrorType::Internal,
            "",
            ErrorAction::None,
            "Unable to Create Store",
        ))?;

        return Ok(Vault { store });
    }

    pub fn store_session(self, token: String) -> Result<(), ErrorPayload> {
        let secret_bucket = self
            .store
            .bucket::<String, String>(Some("secrets"))
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to create Bucket",
            ))?;

        let key = "token".to_string();

        secret_bucket
            .set(&key, &token)
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to Write to Secrets",
            ))?;

        Ok(())
    }

    pub fn get_session(self) -> Result<String, ErrorPayload> {
        let secret_bucket = self
            .store
            .bucket::<String, String>(Some("secrets"))
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to create Bucket",
            ))?;

        let key = String::from("token");

        match secret_bucket
            .get(&key)
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to find Secrets",
            ))? {
            Some(token) => Ok(token),
            None => Err(ErrorPayload {
                action_type: ErrorAction::Redirect,
                details: "No Session Found".to_string(),
                error: "".to_string(),
                error_type: ErrorType::Expected,
                message: "".to_string(),
            }),
        }
    }

    pub fn clear_session(self) -> Result<(), ErrorPayload> {
        let secret_bucket = self
            .store
            .bucket::<String, String>(Some("secrets"))
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to create Bucket",
            ))?;

        let key = String::from("token");

        secret_bucket
            .remove(&key)
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to remove Secrets",
            ))?;

        Ok(())
    }
}
