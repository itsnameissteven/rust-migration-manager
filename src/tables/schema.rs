use crate::sql::db::Schema;
use crate::tables::user::create_user_table;

fn build_schema() -> Schema {
    let tables = vec![create_user_table()];
    Schema::new().tables(tables)
}
