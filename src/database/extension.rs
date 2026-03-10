use std::fmt::Write;

use crate::error::SchemaError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    pub name: String,
}

impl Extension {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn parse(&self) -> Result<String, SchemaError> {
        let mut output = String::from("create extension if not exists ");

        write!(output, "\"{}\";\n", self.name).unwrap();

        Ok(output)
    }
}
