use std::fmt;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Duplicate column value: {0}")]
    ColumnError(String),

    #[error("Duplicate table name value: {0}")]
    TableError(String),

    #[error("Duplicate enum value: {0}")]
    EnumError(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("There was an issue formatting: {0}")]
    FormatError(#[from] fmt::Error),
}

impl From<SchemaError> for io::Error {
    fn from(err: SchemaError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
