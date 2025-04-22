use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub data: Vec<Vec<Value>>,
}

#[derive(Debug, Encode, Decode)]
pub struct Column {
    pub name: String,
    pub data_type: ColumnDataType,
    pub is_primary_key: bool,
    pub is_nullable: bool,
}

#[derive(Debug, Encode, Decode)]
pub enum ColumnDataType {
    Int(Option<u64>),
    Varchar(Option<u64>),
}

#[derive(Debug, Encode, Decode)]
pub enum Value {
    Int(i64),
    Varchar(String),
    Null,
}
