use super::db::controllers::{auth, user, position};
use super::db::DatabaseResponse;
use super::db::models::{user_data::UserDataModel, position::PositionDataModel};

// AUTH COMMANDS | CAUTION
#[tauri::command]
pub async fn register_user(username: String, email: String, password: String, token: String) -> String {
    // cast values
    let password = password.as_bytes();
    let token = token.as_bytes();

    let result = auth::register(username, email, password, token).unwrap();

    if let auth::AuthStatus::Success = result {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn authenticate_user(username: String, password: String) -> String {
    // cast values
    let password = password.as_bytes();

    let result = auth::authenticate(username, password).unwrap();

    println!("invoked");

    if let auth::AuthStatus::Success = result {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn reset_password(email: String, password: String, token: String) -> String {
    // cast values
    let password = password.as_bytes();
    let token = token.as_bytes();

    let result = auth::reset_password(email, password, token).unwrap();

    if let auth::AuthStatus::Success = result {
        "good".into()
    } else {
        "bad".into()
    }
}

// USER COMMANDS | DATA
#[tauri::command]
pub async fn user_check_data(username: String) -> String {
    let result = user::check_data(username).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub fn user_get_data(username: String) -> Result<UserDataModel, String> {
    let result = user::get_data(username).unwrap();
    if result.name.is_empty() {
        Err("Something went wrong".into())
    } else {
        Ok(result)
    }
}

#[tauri::command]
pub fn user_get_users() -> Result<Vec<UserDataModel>, String> {
    let result = user::get_users().unwrap();
    if result.is_empty() {
        Err("Something went wrong".into())
    } else {
        Ok(result)
    }
}

#[tauri::command]
pub async fn user_update_position(username: String, position: String) -> String {
    let result = user::update_position(username, position).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn user_update_status(username: String, status: String) -> String {
    let result = user::update_status(username, status).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn user_update_discord(username: String, discord: String) -> String {
    let result = user::update_discord(username, discord).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn user_update_image(username: String, image: String) -> String {
    let result = user::update_image(username, image).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn user_update(name: String, position: String, status: String, discord: String, image: String) -> String {
    let result = user::update_user(name, position, status, discord, image).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

// POSITION COMMANDS | DATA
#[tauri::command]
pub async fn position_check_position(position: String) -> String {
    let result = position::check_position(position).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn position_get_position(position: String) -> Result<PositionDataModel, String> {
    let result = position::get_position(position).unwrap();
    if result.position.is_empty() {
        Err("Something went wrong".into())
    } else {
        Ok(result)
    }
}

#[tauri::command]
pub async fn position_get_all_positions() -> Result<Vec<PositionDataModel>, String> {
    let result = position::get_all_positions().unwrap();
    if result.is_empty() {
        Err("Something went wrong".into())
    } else {
        Ok(result)
    }
}

#[tauri::command]
pub async fn position_new(position: String, level: i32, display_name: String, description: String, color: String) -> String {
    let result = position::new_position(position, level, display_name, description, color).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

#[tauri::command]
pub async fn position_update(position: String, level: i32, display_name: String, description: String, color: String) -> String {
    let result = position::update_position(position, level, display_name, description, color).unwrap();

    if let DatabaseResponse::Success = result  {
        "good".into()
    } else {
        "bad".into()
    }
}

