use crate::validator;

// validate -> generate key -> hash password -> Store DB -> Store creds

#[tauri::command]
pub async fn sign_up(full_name: String, email: String, password: String) -> Result<(), String> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(String::from("Invalid Email or Password"));
    }

    // generate key

    // hash password

    Ok(())
}

// validate -> Fetch DB -> Compare hash -> Store creds

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<(), String> {
    if !validator::validate_email(&email) || !validator::validate_password(&password) {
        return Err(String::from("Invalid Email or Password"));
    }

    //

    Ok(())
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
