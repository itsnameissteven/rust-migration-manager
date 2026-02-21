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
        ],
    }
}
