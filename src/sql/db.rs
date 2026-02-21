#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    tables: Vec<Table>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

// todo!("Add default");
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    name: String,
    data_type: DataType,
    nullable: bool,
    // default: String,
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
}

impl DataType {
    pub fn as_str(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
