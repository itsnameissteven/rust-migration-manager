use crate::database::{DbEnum, utils::BuildEnum};

pub struct Status;

impl BuildEnum for Status {
    fn db_enum() -> DbEnum {
        DbEnum::new("status", vec!["Approved", "Complete", "Awaiting"])
    }
}
