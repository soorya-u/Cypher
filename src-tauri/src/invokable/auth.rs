use crate::{
    cryptography::{encryption::Encryption, hashing::Hashing},
    validator,
};

// validate -> generate key -> hash password -> Store DB -> Store creds

#[tauri::command]
pub async fn sign_up(full_name: String, email: String, password: String) -> Result<(), String> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(format!("Invalid Email or Password"));
    }

    let unique_key = Encryption::generate_unique_key();

    let hasher = Hashing::new();
    let salt = hasher.get_salt();
    let hashed_password = hasher.hash_data(&password);

     // Store creds

    Ok(())
}

// validate -> Fetch DB -> Compare hash -> Store creds

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<bool, String> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(format!("Invalid Email or Password"));
    }

    // Fetch DB
    let salt = String::from("");
    let hash = String::from("");

    let hasher = Hashing::from(&salt);
    if !hasher.verify_hash(&password, &hash) {
        return Err(format!("Password did not match"));
    }

    // Store creds

    Ok(true)
}
