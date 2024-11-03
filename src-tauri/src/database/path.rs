use std::fs;

use directories::ProjectDirs;

pub fn database_path() -> Result<String, String> {
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

    let database_path_str = database_path
        .to_str()
        .ok_or_else(|| String::from("Failed to convert database path to string"))?;

    let db_url = String::from(database_path_str);

    return Ok(db_url);
}
