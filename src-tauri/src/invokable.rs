use crate::validator::{self, validate_password};

// validate -> generate key -> hash password -> Store DB -> Store creds

#[tauri::command]
pub async fn sign_up(full_name: String, email: String, password: String) -> Result<(), String> {
    if !validator::validate_email(&email) || !validate_password(&password) {
        return Err(String::from("Invalid Email or Password"));
    }

    Ok(())
}

// validate -> Fetch DB -> Compare hash -> Store creds

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<(), String> {
    if !validator::validate_email(&email) || !validate_password(&password) {
        return Err(String::from("Invalid Email or Password"));
    }

    //

    Ok(())
}
