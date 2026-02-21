use std::fmt::Write;

pub trait Format {
    fn as_str(&self) -> String;
}

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn create_table(&self) -> String {
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
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    name: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Text,
    Boolean,
    Date,
    Int,
    Int4,
    Char,
    Varchar,
    Time,
    Timestamp,
    Timestampz,
    Enum,
    Uuid,
}

impl Format for DataType {
    fn as_str(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
