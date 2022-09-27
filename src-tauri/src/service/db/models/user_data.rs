#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub struct UserDataModel {
    pub name: String,
    pub position: String,
    pub status: String,
    pub discord: Option<String>,
    pub image: Option<String>
}

pub fn build_user_data(name: String, position: String, status: String, discord: Option<String>, image: Option<String>) -> UserDataModel {
    UserDataModel {
        name,
        position,
        status,
        discord,
        image
    }
}

pub fn get_table_name() -> String {
    "user_data".into()
}

pub fn create_table_query() -> String {
    let table_name = get_table_name();
    let query = format!("CREATE TABLE IF NOT EXISTS {} (
        name VARCHAR(255) NOT NULL,
        position TINYTEXT NOT NULL,
        status TINYTEXT NOT NULL DEFAULT 'offline',
        discord TEXT NULL DEFAULT NULL,
        image TEXT NULL DEFAULT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        PRIMARY KEY (name)
    )", table_name);

    return query;
}
