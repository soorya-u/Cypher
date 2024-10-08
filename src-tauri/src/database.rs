mod path;
mod queries;
mod users;

use path::database_path;
use queries::CREATE_SCHEMA_QUERY;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::result::Result;
use tauri::async_runtime;

pub struct Database {
    pool: Pool<Sqlite>,
    db_url: String,
}

impl Database {
    async fn database_exists(&self) -> bool {
        !Sqlite::database_exists(&self.db_url).await.unwrap_or(false)
    }

    pub async fn new() -> Result<Self, String> {
        let db_url = database_path()?;

        let pool = SqlitePool::connect(&db_url)
            .await
            .expect("Unable to create pool");

        return Ok(Database { pool, db_url });
    }

    pub async fn initialize_database(&self) -> Result<(), String> {
        if self.database_exists().await {
            return Ok(());
        }

        Sqlite::create_database(&self.db_url)
            .await
            .map_err(|e| format!("Failed to create database: {}", e))?;

        sqlx::query(CREATE_SCHEMA_QUERY)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to run schema query: {}", e))?;
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        async_runtime::block_on(async {
            self.pool.close().await;
        });
    }
}
