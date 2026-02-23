use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SchemaError {
    ColumnError(String),
    TableError,
    Io(std::io::Error),
}

// Implement the Display trait for user-facing messages
impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaError::ColumnError(msg) => write!(f, "Duplicate column value: {}", msg),
            SchemaError::TableError => write!(f, "TBD"),
            SchemaError::Io(e) => write!(f, "{e}"),
        }
    }
}

// Implement the Error trait
impl Error for SchemaError {}

impl From<SchemaError> for io::Error {
    fn from(err: SchemaError) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
