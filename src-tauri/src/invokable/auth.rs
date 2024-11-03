use crate::{
    cryptography::{encryption::Encryption, hashing::Hashing, jwt::JWT, vault::Vault},
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

    // Store creds to DB

    let jwt_token = JWT::create_jwt(&unique_key, &email)?;

    let mut vault = Vault::new()?;
    vault.store_session(jwt_token)?;

    Ok(())
}

// validate -> Fetch DB -> Compare hash -> Store creds

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<(), String> {
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

    let jwt_token = JWT::create_jwt(&hash, &email)?;

    let mut vault = Vault::new()?;
    vault.store_session(jwt_token)?;

    Ok(())
}

#[tauri::command]
// Change Return Type to User
pub async fn get_session() -> Result<(), String> {
    let mut vault = Vault::new()?;
    let token = vault.get_session()?;

    // Handle JWT Expiration
    let email = JWT::read_jwt(&token)?;

    // Read User Type from DB

    Ok(())
}

#[tauri::command]
pub fn logout() -> Result<(), String> {
    let mut vault = Vault::new()?;
    vault.clear_session()
}
