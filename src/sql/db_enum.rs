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
}
