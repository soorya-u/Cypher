use std::result::Result;
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[allow(dead_code)]
#[tauri::command]
pub async fn db_init() -> Result<String,String> {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        return Ok(String::from("Created Database"));
    }
    Ok(String::from("db exists"))
}
