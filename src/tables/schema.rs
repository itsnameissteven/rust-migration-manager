use crate::sql::db::Schema;
use crate::tables::user::{create_phrase_table, create_user_table};

pub fn build() -> Schema {
    let tables = vec![create_user_table(), create_phrase_table()];
    Schema::new().tables(tables)
}
