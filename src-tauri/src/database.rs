mod path;
mod queries;
pub mod users;

use path::database_path;
use queries::schema::CREATE_SCHEMA_QUERY;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::result::Result;
use tauri::async_runtime;

use crate::invokable::{ErrorAction, ErrorPayload, ErrorType};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    async fn database_exists(db_url: &str) -> bool {
        Sqlite::database_exists(db_url).await.unwrap_or(false)
    }

    pub async fn new() -> Result<Self, ErrorPayload> {
        let db_url = database_path().map_err(ErrorPayload::from_message_with_closure(
            ErrorType::Internal,
            "",
            ErrorAction::None,
        ))?;

        if !Database::database_exists(&db_url).await {
            Sqlite::create_database(&db_url).await.map_err(
                ErrorPayload::from_error_with_closure(
                    ErrorType::Internal,
                    "",
                    ErrorAction::None,
                    "Failed to create database",
                ),
            )?;
        }

        let pool =
            SqlitePool::connect(&db_url)
                .await
                .map_err(ErrorPayload::from_error_with_closure(
                    ErrorType::Internal,
                    "",
                    ErrorAction::None,
                    "Unable to create pool",
                ))?;

        sqlx::query(CREATE_SCHEMA_QUERY)
            .execute(&pool)
            .await
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Failed to run schema query",
            ))?;

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
