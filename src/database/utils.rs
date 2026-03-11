use crate::{
    database::{DbEnum, Table},
    error::SchemaError,
};

pub trait Format {
    fn as_str(&self) -> String;
}

pub trait Parse {
    fn parse(&self) -> Result<String, SchemaError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Text,
    Boolean,
    Date,
    Int,
    Int4,
    Char,
    Varchar,
    Time,
    Timestamp,
    Timestampz,
    Enum,
    Uuid,
}

impl Format for DataType {
    fn as_str(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

pub trait BuildTable {
    fn table() -> Table;
}

pub trait BuildEnum {
    fn db_enum() -> DbEnum;
}

#[test]
fn should_equal_type() {
    assert_eq!(DataType::Char, DataType::Char)
}
#[test]
fn should_not_equal() {
    assert_ne!(DataType::Char, DataType::Text)
}
