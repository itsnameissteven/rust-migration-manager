use crate::database::{DbEnum, Extension, Parse, Table};
use crate::error::SchemaError;
use crate::postgres_schema::PostgresSchema;
use chrono::Utc;
use clap::Parser;
use sqlx::migrate::MigrateError;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub migration_path: String,
    #[clap(long, env)]
    pub database_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub tables: Vec<Table>,
    pub migration_path: String,
    database_url: String,
    pub enums: Vec<DbEnum>,
    pub extensions: Vec<Extension>,
}

impl Schema {
    pub fn new() -> Self {
        let config = Config::parse();
        Self {
            tables: Vec::new(),
            migration_path: config.migration_path,
            database_url: config.database_url,
            enums: Vec::new(),
            extensions: Vec::new(),
        }
    }
    pub fn table(mut self, table: Table) -> Self {
        self.tables.push(table);
        self
    }
    pub fn db_enum(mut self, db_enum: DbEnum) -> Self {
        self.enums.push(db_enum);
        self
    }
    pub fn extension(mut self, extension: Extension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub async fn check(&self) -> Result<(), sqlx::migrate::MigrateError> {
        PostgresSchema::connect(&self.database_url)
            .await?
            .get_tables()
            .await;
        Ok(())
    }

    pub fn write(&self, file_name: &str) -> Result<(), SchemaError> {
        let time_stamp = Utc::now().timestamp().to_string();
        let file_path = format!(
            "{}/{}_{}.sql",
            self.migration_path,
            time_stamp,
            file_name.trim()
        );

        match fs::create_dir(&self.migration_path) {
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

    pub async fn migrate(&self, file_name: &str) -> Result<(), sqlx::migrate::MigrateError> {
        let res = self.write(file_name);
        match res {
            Ok(_) => {
                println!("Ok");
                PostgresSchema::connect(&self.database_url)
                    .await?
                    .migrate(&self.migration_path)
                    .await
            }
            Err(_) => {
                println!("");
                Err(MigrateError::ForceNotSupported)
            }
        }
    }

    fn parse(&self) -> Result<String, SchemaError> {
        self.validate_values()?;
        let mut output = String::from("-- Migration --\n");

        for extension in &self.extensions {
            let val = extension.parse()?;
            write!(output, "\n{}", val).unwrap();
        }

        for db_enum in &self.enums {
            let val = db_enum.parse()?;
            write!(output, "\n{}", val).unwrap();
        }

        for table in &self.tables {
            let val = table.parse()?;
            write!(output, "\n{}", val).unwrap();
        }
        Ok(output)
    }
    fn validate_values(&self) -> Result<(), SchemaError> {
        let mut enum_names: HashSet<&String> = HashSet::new();
        let mut table_names: HashSet<&String> = HashSet::new();
        for e in &self.enums {
            if !enum_names.insert(&e.name) {
                return Err(SchemaError::EnumError(e.name.to_string()));
            };
        }
        for t in &self.tables {
            if !table_names.insert(&t.name) {
                return Err(SchemaError::TableError(t.name.to_string()));
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::utils::BuildEnum;
    use crate::database::{BuildTable, Schema};
    use crate::tables::{Status, User};

    #[test]
    fn should_be_ok() {
        let schema = Schema::new()
            .table(User::table())
            .db_enum(Status::db_enum());
        assert!(schema.parse().is_ok());
    }

    #[test]
    fn should_not_be_ok() {
        let schema = Schema::new()
            .db_enum(Status::db_enum())
            .db_enum(Status::db_enum());
        assert!(schema.parse().is_err());
        let schema2 = Schema::new().table(User::table()).table(User::table());
        assert!(schema2.parse().is_err());
    }
}

// SELECT
//   table_name,
//   column_name,
//   data_type,
//   is_nullable,
//   column_default,
//   ordinal_position
// FROM information_schema.columns
// WHERE table_schema = 'public'
// ORDER BY table_name, ordinal_position;
