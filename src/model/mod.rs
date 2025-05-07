use bincode::{Decode, Encode};
use std::cmp::Ordering;
use std::fmt;

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

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Varchar(String),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Varchar(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "NULL"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Varchar(a), Value::Varchar(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),

            // Null is considered less than everything else
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Null, _) => Some(Ordering::Less),
            (_, Value::Null) => Some(Ordering::Greater),

            // Different types are compared by their variant order
            _ => match (self, other) {
                (Value::Int(_), _) => Some(Ordering::Less),
                (_, Value::Int(_)) => Some(Ordering::Greater),
                (Value::Varchar(_), _) => Some(Ordering::Less),
                (_, Value::Varchar(_)) => Some(Ordering::Greater),
                _ => None,
            },
        }
    }
}

impl Clone for Column {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            data_type: match &self.data_type {
                ColumnDataType::Int(len) => ColumnDataType::Int(len.clone()),
                ColumnDataType::Varchar(len) => ColumnDataType::Varchar(len.clone()),
            },
            is_primary_key: self.is_primary_key,
            is_nullable: self.is_nullable,
        }
    }
}
