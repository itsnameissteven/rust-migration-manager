use crate::sql::utils::BuildEnum;
use crate::sql::{BuildTable, Schema};
use crate::tables::{Status, User};

pub fn build() -> Schema {
    Schema::new()
        .table(User::table())
        .db_enum(Status::db_enum())
}
