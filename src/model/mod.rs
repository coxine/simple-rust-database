/// 数据模型模块
///
/// 包含数据库中使用的基本数据结构定义，如列、数据类型和值类型。
/// 这些结构被序列化和反序列化以支持数据持久化。
use bincode::{Decode, Encode};
use std::cmp::Ordering;
use std::fmt;

/// 表列定义结构
///
/// 表示数据库表中的一列，包含列名、数据类型、主键标志和可空标志。
#[derive(Debug, Encode, Decode)]
pub struct Column {
    /// 列名
    pub name: String,
    /// 列的数据类型
    pub data_type: ColumnDataType,
    /// 是否为主键
    pub is_primary_key: bool,
    /// 是否可为 NULL
    pub is_nullable: bool,
}

/// 列数据类型枚举
///
/// 支持整数和可变长度字符串两种基本类型，均可指定可选的长度限制。
#[derive(Debug, Encode, Decode)]
pub enum ColumnDataType {
    /// 整数类型，可选的长度限制
    Int(Option<u64>),
    /// 字符串类型，可选的长度限制
    Varchar(Option<u64>),
}

/// 值类型枚举
///
/// 表示数据库中存储的实际值，支持整数、字符串、布尔值和 NULL。
#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub enum Value {
    /// 整数值
    Int(i64),
    /// 字符串值
    Varchar(String),
    /// 布尔值
    Bool(bool),
    /// NULL 值
    Null,
}

impl fmt::Display for Value {
    /// 实现值的字符串表示
    ///
    /// 用于在 CLI 中显示查询结果时格式化值。
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
    /// 实现值的部分排序
    ///
    /// 用于比较值大小，支持 ORDER BY 等操作。
    /// NULL 被视为小于任何非 NULL 值，不同类型间按类型顺序比较。
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
    /// 实现列的克隆
    ///
    /// 由于 Column 结构包含枚举，手动实现 Clone 以确保正确复制。
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
