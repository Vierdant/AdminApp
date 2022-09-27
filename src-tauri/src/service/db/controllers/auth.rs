use crate::service::db::{self, models::user::build_user, models::user_data::build_user_data};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use mysql::*;
use mysql::prelude::*;



pub enum AuthStatus {
    Success,
    Interrupted,
}

pub fn authenticate(username: String, password: &[u8]) -> Result<AuthStatus> {
    let mut connection = db::get_connection().unwrap();

    let db_password = connection.exec_first::<String, _, _>(
        r"SELECT password FROM user_auth WHERE name = :username", 
        params! {
            "username" => &username,
        }
    )?;

    // user not found
    if db_password.is_none() {
        return Ok(AuthStatus::Interrupted);
    }

    // verify token
    let db_password = db_password.unwrap();
    let parsed_password = PasswordHash::new(&db_password).unwrap();
    if !Argon2::default().verify_password(password, &parsed_password).is_ok() {
        return Ok(AuthStatus::Interrupted);
    }

    Ok(AuthStatus::Success)
}

pub fn register(username: String, email: String, password: &[u8], token: &[u8]) -> Result<AuthStatus> {
    let mut connection = db::get_connection().unwrap();
    
    // valid row
    // if email is "input" that means that registeration row is not used
    let db_email = connection.exec_first::<String, _, _>(
        r"SELECT email FROM user_auth WHERE name = :username", 
        params! {
            "username" => &username,
        }
    )?;

    // user not found
    if db_email.is_none() {
        return Ok(AuthStatus::Interrupted);
    }

    if db_email.unwrap() != "input" {
        return Ok(AuthStatus::Interrupted);
    }
    
    // ensure token
    let db_token = connection.exec_first::<String, _, _>(
        r"SELECT password FROM user_auth WHERE name = :username", 
        params! {
            "username" => &username,
        }
    )?.unwrap();

    // verify token
    let parsed_token = PasswordHash::new(&db_token).unwrap();
    if !Argon2::default().verify_password(token, &parsed_token).is_ok() {
        return Ok(AuthStatus::Interrupted);
    }

    // encrypt password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap();
    let stringified_hash = password_hash.to_string();

    let name = username.clone();
    // build model
    let user = build_user(
        username,
        email,
        stringified_hash,
    );

    //update database
    connection.exec_first::<String, _, _>(
        r"UPDATE user_auth SET name = :name, email = :email, password = :password WHERE name = :username", 
        params! {
            "name" => user.name,
            "email" => user.email,
            "password" => user.password,
            "username" => &name,
        }
    )?;

    // build user data & user profile
    create_user_data_entry(name, "guest".to_owned(), "offline".to_owned()).unwrap();

    Ok(AuthStatus::Success)
}

pub fn reset_password(email: String, password: &[u8], token: &[u8]) -> Result<AuthStatus> {
    let mut connection = db::get_connection().unwrap();
    
    // valid row
    // if email is not "input" that means that registeration row is not used
    let db_token = connection.exec_first::<String, _, _>(
        r"SELECT password FROM user_auth WHERE email = :email", 
        params! {
            "email" => &email,
        }
    )?;

    // user not found
    if db_token.is_none() {
        println!("User not found!");
        return Ok(AuthStatus::Interrupted);
    }

    // verify token
    let db_token = db_token.unwrap();
    let parsed_token = PasswordHash::new(&db_token).unwrap();
    if !Argon2::default().verify_password(token, &parsed_token).is_ok() {
        return Ok(AuthStatus::Interrupted);
    }

    // encrypt password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap();
    let stringified_hash = password_hash.to_string();

    //update database
    connection.exec_first::<String, _, _>(
        r"UPDATE user_auth SET password = :password WHERE email = :email", 
        params! {
            "password" => stringified_hash,
            "email" => email,
        }
    )?;

    Ok(AuthStatus::Success)
}

fn create_user_data_entry(name: String, position: String, status: String) -> Result<()> {
    let mut connection = db::get_connection().unwrap();

    let user_data = build_user_data(
        name,
        position,
        status,
        // placeholder to avoid error
        None,
        None
    );

    connection.exec_first::<String, _, _>(
        r"INSERT INTO user_data (name, position, status) VALUES (:name, :position, :status)", 
        params! {
            "name" => user_data.name,
            "position" => user_data.position,
            "status" => user_data.status,
        }
    )?;

    Ok(())
}