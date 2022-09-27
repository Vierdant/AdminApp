use mysql::{prelude::*, PooledConn};

pub mod user;
pub mod user_data;
pub mod position;

pub fn ensure_all_models(mut connection: PooledConn) {

    let auth_query = user::create_table_query();
    let data_query = user_data::create_table_query();
    let position_query = position::create_table_query();

    connection.query_drop(auth_query).unwrap();
    connection.query_drop(data_query).unwrap();
    connection.query_drop(position_query).unwrap();
}