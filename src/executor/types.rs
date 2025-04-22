use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub data: Vec<Vec<Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: ColumnDataType,
    pub is_primary_key: bool,
    pub is_nullable: bool,
}

#[derive(Serialize, Deserialize)]
pub enum ColumnDataType {
    Int(Option<u64>),
    Varchar(Option<u64>),
}

#[derive(Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Varchar(String),
    Null,
}
