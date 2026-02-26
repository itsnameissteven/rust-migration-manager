use crate::database::utils::BuildEnum;
use crate::database::{BuildTable, Schema};
use crate::tables::{Status, User};

pub fn build() -> Schema {
    Schema::new()
        .table(User::table())
        .db_enum(Status::db_enum())
}
