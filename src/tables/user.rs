use crate::sql::{BuildTable, Column, Table};

pub struct User;

impl BuildTable for User {
    fn table() -> Table {
        Table::new("user")
            .column(Column::id("user_id").default("uuid"))
            .column(Column::text("first_name"))
            .column(Column::text("last_name"))
            .with_time_stamps()
    }
}
