use crate::error::SchemaError;
use crate::sql::Table;
use chrono::Utc;
use std::fmt::Write;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    migration_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub tables: Vec<Table>,
    pub config: Config,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            config: Config {
                migration_path: String::from("migrations"),
            },
        }
    }
    pub fn tables(mut self, tables: Vec<Table>) -> Self {
        self.tables = tables;
        self
    }
    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
    pub fn migrate(&self, file_name: &str) -> Result<(), SchemaError> {
        let time_stamp = Utc::now().timestamp().to_string();
        let file_path = format!(
            "{}/{}_{}.sql",
            self.config.migration_path,
            time_stamp,
            file_name.trim()
        );

        match fs::create_dir(&self.config.migration_path) {
            Ok(_) => {
                println!("Migration directory created");
                Ok(())
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(SchemaError::Io(e)),
        }?;

        let contents = self.parse()?;

        match fs::write(file_path, contents) {
            Ok(_) => {
                println!("Migration created");
                Ok(())
            }
            Err(e) => Err(SchemaError::Io(e)),
        }
    }

    fn parse(&self) -> Result<String, SchemaError> {
        let mut output = String::from("-- Migration --");

        for table in &self.tables {
            let val = table.parse()?;
            write!(output, "\n\n{}", val).unwrap();
            //     return Err(t);
            // } else if let Ok(val) = res {
            //     write!(output, "\n\n{}", val).unwrap();
            // }
        }
        Ok(output)
    }
}
