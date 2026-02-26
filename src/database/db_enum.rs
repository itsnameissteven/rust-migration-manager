use std::fmt::Write;

use crate::error::SchemaError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbEnum {
    pub name: String,
    pub values: Vec<String>,
}

impl DbEnum {
    pub fn new<T: Into<String>>(name: impl Into<String>, values: Vec<T>) -> Self {
        Self {
            name: name.into(),
            values: values.into_iter().map(Into::into).collect(),
        }
    }
    pub fn parse(&self) -> Result<String, SchemaError> {
        let mut output = String::from("CREATE TYPE ");

        if self.values.len() == 0 {
            return Err(SchemaError::EnumError(format!(
                "No enum values provided for {}",
                self.name
            )));
        }
        write!(output, "'{}' AS ENUM (", self.name).unwrap();

        let values: String = self
            .values
            .iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<_>>()
            .join(", ");

        write!(output, "{});\n", values).unwrap();

        Ok(output)
    }
}
