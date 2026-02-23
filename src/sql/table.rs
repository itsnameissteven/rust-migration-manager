use crate::error::SchemaError;
use crate::sql::{Column, Format};
use std::collections::HashSet;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
        }
    }
    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }
    pub fn with_time_stamps(self) -> Self {
        self.column(Column::time_stamp("created_at"))
            .column(Column::time_stamp("updated_at"))
    }
    pub fn parse(&self) -> Result<String, SchemaError> {
        if let Err(e) = self.validate_col_names() {
            return Err(e);
        }
        let mut output = String::from("CREATE TABLE ");
        let len = self.columns.len();

        write!(output, "\"{}\" \n(", self.name).unwrap();

        for (i, col) in self.columns.iter().enumerate() {
            write!(
                output,
                "\n {}{}",
                col.as_str(),
                if len - 1 == i { "" } else { "," }
            )
            .unwrap();
        }

        write!(output, "\n);").unwrap();
        Ok(output)
    }

    fn validate_col_names(&self) -> Result<(), SchemaError> {
        let mut column_names: HashSet<&String> = HashSet::new();
        for col in &self.columns {
            if column_names.contains(&col.name) {
                return Err(SchemaError::ColumnError(col.name.to_string()));
            };
            column_names.insert(&col.name);
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sql::utils::DataType;

    #[test]
    fn should_equal() {
        let table1 = Table {
            name: String::from("user"),
            columns: vec![
                Column::new("user_id", DataType::Uuid)
                    .unique()
                    .primary()
                    .default("uuid"),
                Column::new("first_name", DataType::Text),
                Column::new("last_name", DataType::Text),
                Column::new("created_at", DataType::Timestamp).default("now()"),
                Column::new("updated_at", DataType::Timestamp).default("now()"),
            ],
        };
        let table2 = Table {
            name: String::from("user"),
            columns: vec![
                Column::new("user_id", DataType::Uuid)
                    .unique()
                    .primary()
                    .default("uuid"),
                Column::new("first_name", DataType::Text),
                Column::new("last_name", DataType::Text),
                Column::new("created_at", DataType::Timestamp).default("now()"),
                Column::new("updated_at", DataType::Timestamp).default("now()"),
            ],
        };
        assert!(table1 == table2);
    }
}

// #[test]
// fn should_not_equal() {
//     assert_ne!(
//         Column::new("test", DataType::Text).default(""),
//         Column::new("test", DataType::Text)
//     );
// }
