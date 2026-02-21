use crate::sql::db::Schema;
use crate::tables::user::create_user_table;

fn build_schema() -> Schema {
    Schema {
        tables: vec![create_user_table()],
    }
}
