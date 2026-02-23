use crate::sql::utils::{DataType, Format};
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

impl Format for Column {
    fn as_str(&self) -> String {
        let mut output = String::new();
        write!(output, "{} {}", self.name, self.data_type.as_str()).unwrap();

        if self.is_primary {
            write!(output, "  PRIMARY KEY").unwrap();
        }
        if self.is_unique {
            write!(output, "  UNIQUE").unwrap();
        }
        if !self.is_nullable {
            write!(output, "  NOT NULL").unwrap();
        }
        if let Some(ref default) = self.default {
            write!(output, " DEFAULT {}", default).unwrap();
        }
        output
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
