use directories::ProjectDirs;
// use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::result::Result;

pub async fn initialize_database() -> Result<String, String> {
    let db_file = String::from("sqlite.db");

    let proj_dirs = ProjectDirs::from("dev", "soorya-u", "Cypher").ok_or_else(|| String::from("Failed to get project directories"))?;

    let database_path = proj_dirs.data_local_dir().join(&db_file);

    let database_directory =database_path.to_str().ok_or_else(|| String::from("Failed to convert database path to string"))?;

    let db_url = String::from(database_directory);

    Ok(db_url)

    // if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
    //     Sqlite::create_database(&db_url).await.unwrap();
    //     return Ok(String::from("Created Database"));
    // }
    // Ok(String::from("Database exists"))
}