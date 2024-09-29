mod queries;

use directories::ProjectDirs;
use queries::CREATE_SCHEMA_QUERY;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::{fs, result::Result};

pub async fn initialize_database() -> Result<String, String> {
    let db_file = String::from("root.db");

    let proj_dirs = ProjectDirs::from("dev", "soorya-u", "cypher")
        .ok_or_else(|| String::from("Failed to get project directories"))?;

    let database_path = proj_dirs.data_local_dir().join(&db_file);
    let parent_dir = database_path
        .parent()
        .ok_or_else(|| String::from("Failed to get parent directory"))?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)
            .map_err(|_| String::from("Failed to create database directory"))?;
    }

    let database_directory = database_path
        .to_str()
        .ok_or_else(|| String::from("Failed to convert database path to string"))?;

    let db_url = String::from(database_directory);
    println!("Database URL: {}", db_url.as_str());

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        let pool = SqlitePool::connect(&db_url)
            .await
            .expect("Unable to create pool");
        sqlx::query(CREATE_SCHEMA_QUERY)
            .execute(&pool)
            .await
            .expect("Unable to run query");
        pool.close().await;

        return Ok(String::from("Schema Creation Successfull"));
    }
    Ok(String::from("Database exists"))
}
