use crate::sql::{BuildTable, Schema};
use crate::tables::User;

pub fn build() -> Schema {
    let tables = vec![User::table()];
    Schema::new().tables(tables)
}
