use crate::{
    cryptography::{encryption::Encryption, hashing::Hashing, jwt::JWT, vault::Vault},
    database::users::User,
    invokable::ErrorPayload,
    validator,
};

#[tauri::command]
pub async fn sign_up(full_name: String, email: String, password: String) -> Result<(), String> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(format!("Invalid Email or Password"));
    }

    let unique_key = Encryption::generate_unique_key();

    let hasher = Hashing::new();
    let salt = hasher.get_salt();
    let hashed_password = hasher.hash_data(&password);

    let user = User::new(&full_name, &email, &hashed_password, &unique_key, &salt);

    user.insert_user_to_db().await?;

    let jwt_token = JWT::create_jwt(&unique_key, &email)?;

    let vault = Vault::new()?;
    vault.store_session(jwt_token)?;

    Ok(())
}

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<User, String> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(format!("Invalid Email or Password"));
    }

    let user = User::from_email(&email).await?;

    let salt = &user.hash_salt;
    let hash = &user.hashed_password;
    let secret = &user.unique_key;

    let hasher = Hashing::from(salt);
    if !hasher.verify_hash(&password, hash) {
        return Err(format!("Password did not match"));
    }

    let jwt_token = JWT::create_jwt(secret, &email)?;

    let vault = Vault::new()?;
    vault.store_session(jwt_token)?;

    Ok(user)
}

#[tauri::command]
pub async fn get_session() -> Result<User, String> {
    let vault: Vault = Vault::new()?;
    let token = vault.get_session()?;

    if JWT::is_jwt_expired(&token)? {
        return Err(format!("Jwt has been Expired"));
    }

    let email = JWT::read_jwt(&token)?;
    let user = User::from_email(&email).await?;

    Ok(user)
}

#[tauri::command]
pub fn logout() -> Result<(), ErrorPayload> {
    let vault: Vault = Vault::new().map_err(ErrorPayload::from_message_with_closure(""))?;
    vault.clear_session().map_err(ErrorPayload::from_message_with_closure(""))
}
