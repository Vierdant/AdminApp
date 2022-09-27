use crate::service::db::{self, DatabaseResponse, models::position::PositionDataModel};

use mysql::*;
use mysql::prelude::*;

pub fn check_position(position: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    let db_position = connection.exec_first::<String, _, _>(
        r"SELECT position FROM positions WHERE position = :position", 
        params! {
            "position" => &position,
        }
    )?;

    // user not found
    if db_position.is_none() {
        return Ok(DatabaseResponse::Error);
    }

    Ok(DatabaseResponse::Success)
}

pub fn get_position(position: String) -> Result<PositionDataModel> {
    let mut connection = db::get_connection().unwrap();
    // assumes that data is ensured
    let mut db_data = connection.query_map(
        format!("SELECT * FROM positions WHERE position = {}", position),
        |(position, level, display_name, description, color)| {
            PositionDataModel {
                position,
                level,
                display_name,
                description,
                color
            }
        }).unwrap();
    
    Ok(db_data.pop().unwrap())
}

pub fn get_all_positions() -> Result<Vec<PositionDataModel>> {
    let mut connection = db::get_connection().unwrap();
    // assumes that data is ensured
    let db_data = connection.query_map(
        format!("SELECT * FROM positions"),
        |(position, level, display_name, description, color)| {
            PositionDataModel {
                position,
                level,
                display_name,
                description,
                color
            }
        }).unwrap();
    
    Ok(db_data)
}

pub fn new_position(position: String, level:i32, display_name: String, description: String, color: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"INSERT INTO positions (position, level, display_name, description, color) VALUES (:position, :level, :display_name, :description, :color)", 
        params! {
            "position" => &position,
            "level" => &level,
            "display_name" => &display_name,
            "description" => &description,
            "color" => &color,
        }
    )?;

    Ok(DatabaseResponse::Success)
}

pub fn update_position(position: String, level: i32, display_name: String, description: String, color: String) -> Result<DatabaseResponse> {
    let mut connection = db::get_connection().unwrap();

    connection.exec_drop(
        r"UPDATE positions SET display_name = :display_name, level = :level, description = :description, color = :color WHERE position = :position", 
        params! {
            "position" => &position,
            "level" => &level,
            "display_name" => &display_name,
            "description" => &description,
            "color" => &color,
        }
    )?;

    Ok(DatabaseResponse::Success)
}