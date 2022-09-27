pub mod models;
pub mod controllers;

use mysql::*;
use dotenv::dotenv;
use std::env;

pub enum DatabaseResponse {
    Success,
    Error,
}

pub fn ensure_models() {
    let connection = get_connection().unwrap();
    // ensure tables
    models::ensure_all_models(connection);
}

pub fn get_connection() -> Result<PooledConn> {
    dotenv().ok();

    println!("Connecting to database...");

    let host = env::var("ADB_HOST").expect("ADB_HOST must be set");
    let user = env::var("ADB_USER").expect("ADB_USER must be set");
    let pass = env::var("ADB_PASS").expect("ADB_PASS must be set");
    let db_name = env::var("ADB_DB").expect("ADB_DB must be set");

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .user(Some(user))
        .pass(Some(pass))
        .db_name(Some(db_name))
        .prefer_socket(false)
        .tcp_port(3306);

    let pool = Pool::new(opts)?;

    let connection = pool.get_conn()?;

    Ok(connection)
}