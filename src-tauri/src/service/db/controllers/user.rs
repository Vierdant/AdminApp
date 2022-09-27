use crate::service::db::{self, DatabaseResponse, models::user_data::UserDataModel};

use mysql::*;
use mysql::prelude::*;

pub fn check_data(name: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    let db_name = connection.exec_first::<String, _, _>(
        r"SELECT name FROM user_data WHERE name = :name", 
        params! {
            "name" => &name,
        }
    )?;

    // user not found
    if db_name.is_none() {
        return Ok(DatabaseResponse::Error);
    }

    Ok(DatabaseResponse::Success)
}

pub fn get_data(username: String) -> Result<UserDataModel> {
    let mut connection = db::get_connection().unwrap();
    // assumes that data is ensured
    let mut db_data = connection.query_map(
        format!("SELECT name, position, status, discord, image FROM user_data WHERE name = '{}'", username),
        |(name, position, status, discord, image)| {
            UserDataModel {
                name,
                position, 
                status,
                discord,
                image
            }
        }).unwrap();
    
    Ok(db_data.pop().unwrap())
}

pub fn get_users() -> Result<Vec<UserDataModel>> {
    let mut connection = db::get_connection().unwrap();
    // assumes that data is ensured
    let db_data = connection.query_map(
        format!("SELECT * FROM user_data"),
        |(name, position, status, discord, image)| {
            UserDataModel {
                name,
                position, 
                status,
                discord,
                image
            }
        }).unwrap();
    
    Ok(db_data)
}

pub fn update_position(name: String, position: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"UPDATE user_data SET position = :position WHERE name = :name", 
        params! {
            "name" => &name,
            "position" => &position,
        }
    )?;

    Ok(DatabaseResponse::Success)
}

pub fn update_status(name: String, status: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"UPDATE user_data SET status = :status WHERE name = :name", 
        params! {
            "name" => &name,
            "status" => &status,
        }
    )?;

    Ok(DatabaseResponse::Success)
}

pub fn update_discord(name: String, discord: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"UPDATE user_data SET discord = :discord WHERE name = :name", 
        params! {
            "name" => &name,
            "discord" => &discord,
        }
    )?;

    Ok(DatabaseResponse::Success)
}

pub fn update_image(name: String, image: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"UPDATE user_data SET image = :image WHERE name = :name", 
        params! {
            "name" => &name,
            "image" => &image,
        }
    )?;

    Ok(DatabaseResponse::Success)
}

pub fn update_user(name: String, position: String, status: String, discord: String, image: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"UPDATE user_data SET position = :position, status = :status, discord = :discord, image = :image WHERE name = :name", 
        params! {
            "name" => &name,
            "position" => &position,
            "status" => &status,
            "discord" => &discord,
            "image" => &image,
        }
    )?;

    Ok(DatabaseResponse::Success)
}