pub trait Format {
    fn as_str(&self) -> String;
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

#[test]
fn should_equal_type() {
    assert_eq!(DataType::Char, DataType::Char)
}
#[test]
fn should_not_equal() {
    assert_ne!(DataType::Char, DataType::Text)
}
