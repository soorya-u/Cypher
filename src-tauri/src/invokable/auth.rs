use crate::{
    cryptography::{encryption::Encryption, hashing::Hashing, jwt::JWT, vault::Vault},
    database::users::User,
    validator,
};

use super::{ErrorAction, ErrorPayload, ErrorType, IpcUser};

#[tauri::command]
pub async fn sign_up(
    full_name: String,
    email: String,
    password: String,
) -> Result<IpcUser, ErrorPayload> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(ErrorPayload {
            action_type: ErrorAction::Redirect,
            details: "regex did not match".to_string(),
            error: "".to_string(),
            error_type: ErrorType::User,
            message: "Email or Password does not match pattern.".to_string(),
        });
    }

    let unique_key = Encryption::generate_unique_key();

    let hasher = Hashing::new();
    let salt = hasher.get_salt();
    let hashed_password = hasher.hash_data(&password);

    let user = User::new(&full_name, &email, &hashed_password, &unique_key, &salt);

    user.insert_user_to_db().await?;

    let jwt_token = JWT::create_jwt(&email)?;

    let vault = Vault::new()?;
    vault.store_session(jwt_token)?;

    Ok(user.to_ipc_user())
}

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<IpcUser, ErrorPayload> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(ErrorPayload {
            action_type: ErrorAction::Redirect,
            details: "regex did not match".to_string(),
            error: "".to_string(),
            error_type: ErrorType::User,
            message: "Email or Password does not match pattern.".to_string(),
        });
    }

    let user = User::from_email(&email).await?;

    let salt = &user.hash_salt;
    let hash = &user.hashed_password;

    let hasher = Hashing::from(salt);
    if !hasher.verify_hash(&password, hash) {
        return Err(ErrorPayload {
            action_type: ErrorAction::Redirect,
            details: "regex did not match".to_string(),
            error: "".to_string(),
            error_type: ErrorType::User,
            message: "Email or Password is incorrect!".to_string(),
        });
    }

    let jwt_token = JWT::create_jwt(&email)?;

    let vault = Vault::new()?;
    vault.store_session(jwt_token)?;

    Ok(user.to_ipc_user())
}

#[tauri::command]
pub async fn get_session() -> Result<IpcUser, ErrorPayload> {
    let vault: Vault = Vault::new()?;
    let token = vault.get_session()?;

    let email = JWT::decode_jwt(&token)?;
    let user = User::from_email(&email).await?;

    Ok(user.to_ipc_user())
}

#[tauri::command]
pub fn logout() -> Result<(), ErrorPayload> {
    let vault: Vault = Vault::new()?;
    vault.clear_session()
}
