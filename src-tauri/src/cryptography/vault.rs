use directories::ProjectDirs;
use kv::{Config, Store};
use std::fs::{self, File};

use crate::invokable::ErrorPayload;

pub struct Vault {
    store: Store,
}

impl Vault {
    pub fn new() -> Result<Vault, String> {
        let secrets_file = String::from("session.secrets");

        let proj_dirs = ProjectDirs::from("dev", "soorya-u", "cypher")
            .ok_or_else(|| String::from("Failed to get project directories"))?;

        let secrets_path = proj_dirs.data_local_dir().join(&secrets_file);
        let parent_dir = secrets_path
            .parent()
            .ok_or_else(|| String::from("Failed to get parent directory"))?;
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)
                .map_err(|_| String::from("Failed to create database directory"))?;
        }

        if !parent_dir.exists() {
            File::create(secrets_path.clone()).expect("Unable to create secrets file");
        }

        let cnf = Config::new(secrets_path);
        let store = Store::new(cnf).expect("Unable to Create Store");

        return Ok(Vault { store });
    }

    pub fn store_session(self, token: String) -> Result<(), String> {
        let secret_bucket = self
            .store
            .bucket::<String, String>(Some("secrets"))
            .expect("Unable to create Bucket");

        let key = String::from("token");

        secret_bucket
            .set(&key, &token)
            .expect("Unable to Write to Secrets");

        Ok(())
    }

    pub fn get_session(self) -> Result<String, String> {
        let secret_bucket = self
            .store
            .bucket::<String, String>(Some("secrets"))
            .expect("Unable to create Bucket");

        let key = String::from("token");

        // TODO: Handle Differently with ErrorPayload as return value
        match secret_bucket.get(&key).expect("Unable to find Secrets") {
            Some(token) => Ok(token),
            None => Ok(String::from("")),
        }
    }

    pub fn clear_session(self) -> Result<(), String> {
        let secret_bucket = self
            .store
            .bucket::<String, String>(Some("secrets"))
            .expect("Unable to create Bucket");

        let key = String::from("token");

        secret_bucket
            .remove(&key)
            .expect("Unable to remove Secrets");

        Ok(())
    }
}
