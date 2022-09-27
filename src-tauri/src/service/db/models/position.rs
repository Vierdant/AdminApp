#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub struct PositionDataModel {
    pub position: String, // Position name
    pub level: i32, // Position name
    pub display_name: String, // Position display name shown in pages
    pub description: Option<String>, // Position description
    pub color: String, // Position color
}

// there was no need for a build model as it was a simple table

pub fn get_table_name() -> String {
    "positions".into()
}

pub fn create_table_query() -> String {
    let table_name = get_table_name();
    let query = format!("CREATE TABLE IF NOT EXISTS {} (
        position VARCHAR(255) NOT NULL,
        level INT NOT NULL DEFAULT 1,
        display_name VARCHAR(255) NOT NULL,
        description LONGTEXT NULL DEFAULT NULL,
        color VARCHAR(255) NOT NULL DEFAULT '#000000',
        PRIMARY KEY (position)
    )", table_name);

    return query;
}