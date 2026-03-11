use crate::database::{DataType, Format};
use crate::prelude::*;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    data_type: DataType,
    is_unique: bool,
    is_nullable: bool,
    is_primary: bool,
    default: Option<String>,
}

impl Column {
    pub fn new(name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            name: name.into(),
            data_type,
            is_unique: false,
            is_nullable: false,
            is_primary: false,
            default: None,
        }
    }

    pub fn unique(mut self) -> Self {
        self.is_unique = true;
        self
    }

    pub fn nullable(mut self) -> Self {
        self.is_nullable = true;
        self
    }

    pub fn primary(mut self) -> Self {
        self.is_primary = true;
        self
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }
}

impl Column {
    pub fn id(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Uuid).unique().primary()
    }
    pub fn text(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Text)
    }
    pub fn time_stamp(name: impl Into<String>) -> Self {
        Self::new(name, DataType::Timestamp).default("now()")
    }
}

impl Parse for Column {
    fn parse(&self) -> Result<String, SchemaError> {
        let mut output = String::new();
        write!(output, "{} {}", self.name, self.data_type.as_str())?;

        if self.is_primary {
            write!(output, "  PRIMARY KEY")?;
        }
        if self.is_unique {
            write!(output, "  UNIQUE")?;
        }
        if !self.is_nullable {
            write!(output, "  NOT NULL")?;
        }
        if let Some(ref default) = self.default {
            write!(output, " DEFAULT {}", default)?;
        }
        Ok(output)
    }
}

#[test]
fn should_equal() {
    assert_eq!(
        Column::new("test", DataType::Text),
        Column::new("test", DataType::Text)
    );
}

#[test]
fn should_not_equal() {
    assert_ne!(
        Column::new("test", DataType::Text).default(""),
        Column::new("test", DataType::Text)
    );
}
