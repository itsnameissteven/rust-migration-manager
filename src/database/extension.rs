use crate::error::SchemaError;
use crate::prelude::*;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    pub name: String,
}

impl Extension {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Parse for Extension {
    fn parse(&self) -> Result<String, SchemaError> {
        let mut output = String::from("create extension if not exists ");

        write!(output, "\"{}\";\n", self.name)?;

        Ok(output)
    }
}
