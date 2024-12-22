mod path;
mod queries;
pub mod users;

use path::database_path;
use queries::schema::CREATE_SCHEMA_QUERY;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::result::Result;
use tauri::async_runtime;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    async fn database_exists(db_url: &str) -> bool {
        Sqlite::database_exists(db_url).await.unwrap_or(false)
    }

    pub async fn new() -> Result<Self, String> {
        let db_url = database_path()?;

        if !Database::database_exists(&db_url).await {
            Sqlite::create_database(&db_url)
                .await
                .map_err(|e| format!("Failed to create database: {}", e))?;
        }

        let pool = SqlitePool::connect(&db_url)
            .await
            .expect("Unable to create pool");

        sqlx::query(CREATE_SCHEMA_QUERY)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to run schema query: {}", e))?;

        Ok(Database { pool })
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let pool = self.pool.clone();
        async_runtime::spawn(async move {
            pool.close().await;
        });
    }
}
