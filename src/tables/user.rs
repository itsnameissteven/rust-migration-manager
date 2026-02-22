use crate::sql::db::{Column, DataType, Table};

pub fn create_user_table() -> Table {
    Table {
        name: String::from("user"),
        columns: vec![
            Column::new("user_id", DataType::Uuid)
                .unique()
                .primary()
                .default("uuid"),
            Column::new("first_name", DataType::Text),
            Column::new("last_name", DataType::Text),
            Column::new("created_at", DataType::Timestamp).default("now()"),
            Column::new("updated_at", DataType::Timestamp).default("now()"),
        ],
    }
}

pub fn create_phrase_table() -> Table {
    Table {
        name: String::from("phrase"),
        columns: vec![
            Column::new("phrase_id", DataType::Uuid)
                .unique()
                .primary()
                .default("uuid"),
            Column::new("desc", DataType::Text),
        ],
    }
}
