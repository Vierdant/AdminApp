pub mod commands;

mod db;

pub fn initiate_db() {
    db::ensure_models();
}