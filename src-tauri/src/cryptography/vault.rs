use directories::ProjectDirs;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub struct Vault {
    secrets: File,
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
            let secrets =
                File::create(secrets_path.clone()).expect("Unable to create secrets file");
            return Ok(Vault { secrets });
        } else {
            let secrets = File::open(secrets_path.clone()).expect("Unable to Open secrets");
            return Ok(Vault { secrets });
        }
    }

    pub fn store_session(&mut self, token: String) -> Result<(), String> {
        self.secrets
            .write(token.as_bytes())
            .expect("Unable to Write to Secrets");
        Ok(())
    }

    pub fn get_session(&mut self) -> Result<String, String> {
        let mut token = String::new();
        self.secrets
            .read_to_string(&mut token)
            .expect("Unable to read secrets");
        Ok(token)
    }

    pub fn clear_session(&mut self) -> Result<(), String> {
        self.store_session(String::from(""))
    }
}
