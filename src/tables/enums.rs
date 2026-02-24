use crate::sql::{DbEnum, utils::BuildEnum};

pub struct Status;

impl BuildEnum for Status {
    fn db_enum() -> DbEnum {
        DbEnum::new("Status", vec!["Approved", "Complete", "Awaiting"])
    }
}
