use bincode::{Decode, Encode};

use crate::utils::log_info;

use super::ExecutionError;
use sqlparser::ast::{
    Assignment, AssignmentTarget, BinaryOperator as BinOp, Expr, Value as SqlValue,
};

#[derive(Debug, Encode, Decode)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub data: Vec<Vec<Value>>,
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self {
            name,
            columns,
            data: Vec::new(),
        }
    }

    pub fn insert_row(&mut self, values: Vec<Value>) -> Result<(), ExecutionError> {
        self.validate_row(&values)?;
        self.data.push(values);
        Ok(())
    }

    /// Validates that a row of values conforms to the table's schema.
    ///
    /// This method performs the following validations:
    /// - Checks that the number of values matches the number of columns
    /// - Verifies that each value's type matches its corresponding column's type
    /// - Ensures integer and varchar values don't exceed their defined length limits
    /// - Prevents NULL values in non-nullable or primary key columns
    /// - Enforces primary key uniqueness constraints
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values representing a row to be inserted
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all validations pass
    /// * `Err(ExecutionError)` with a detailed error message if any validation fails
    fn validate_row(&self, values: &[Value]) -> Result<(), ExecutionError> {
        if values.len() != self.columns.len() {
            return Err(ExecutionError::TypeUnmatch(format!(
                "插入数据列数不匹配：期望 {}, 实际 {}",
                self.columns.len(),
                values.len()
            )));
        }

        for (i, value) in values.iter().enumerate() {
            let column = &self.columns[i];
            match (value, &column.data_type) {
                (Value::Int(val), ColumnDataType::Int(Some(max_len))) => {
                    if val.to_string().len() > *max_len as usize {
                        return Err(ExecutionError::TypeUnmatch(format!(
                            "列 '{}' 的整数值 {} 超出长度限制 {}",
                            column.name, val, max_len
                        )));
                    }
                }
                (Value::Int(_), ColumnDataType::Int(_)) => {}
                (Value::Varchar(val), ColumnDataType::Varchar(Some(max_len))) => {
                    if val.len() > *max_len as usize {
                        return Err(ExecutionError::TypeUnmatch(format!(
                            "列 '{}' 的字符串值长度 {} 超出限制 {}",
                            column.name,
                            val.len(),
                            max_len
                        )));
                    }
                }
                (Value::Varchar(_), ColumnDataType::Varchar(_)) => {}
                (Value::Null, _) => {
                    if !column.is_nullable || column.is_primary_key {
                        println!("Field '{}' doesn't have a default value", column.name);
                        return Err(ExecutionError::TypeUnmatch(format!(
                            "列 '{}' 不允许 NULL 值",
                            column.name
                        )));
                    }
                }
                _ => {
                    return Err(ExecutionError::TypeUnmatch(format!(
                        "列 '{}' 的值类型不匹配",
                        column.name
                    )));
                }
            }
            if column.is_primary_key {
                if self.is_primary_key_exists(value, column) {
                    println!("Error: Duplicate entry '{}' for key 'PRIMARY'", value);
                    return Err(ExecutionError::PrimaryKeyConflictError(format!(
                        "列 '{}' 的值 '{:?}' 已存在",
                        column.name, value
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn delete_rows(&mut self, where_clause: &Option<Expr>) -> Result<(), ExecutionError> {
        let matching_row_indices = self.filter_rows(where_clause)?;

        // 从后向前删除，避免索引错位
        let mut matching_row_indices = matching_row_indices;
        matching_row_indices.sort_unstable_by(|a, b| b.cmp(a));

        for idx in matching_row_indices.iter() {
            println!("Delete Row {:?}", self.data[*idx]);
            self.data.remove(*idx);
        }

        Ok(())
    }

    pub fn filter_rows(&self, where_clause: &Option<Expr>) -> Result<Vec<usize>, ExecutionError> {
        if where_clause.is_none() {
            // 如果没有 WHERE 子句，返回所有行的索引
            return Ok((0..self.data.len()).collect());
        }
        let expr = where_clause.as_ref().unwrap();
        let mut matching_rows = Vec::new();

        // 遍历所有行，评估 WHERE 表达式
        for (row_idx, row) in self.data.iter().enumerate() {
            match self.evaluate_expr(expr, row) {
                Ok(Value::Bool(true)) => matching_rows.push(row_idx),
                Ok(Value::Bool(false)) => {}
                Ok(_) => {
                    return Err(ExecutionError::ExecutionError(
                        "筛选条件必须可判断的表达式".to_string(),
                    ))
                }
                Err(e) => return Err(e),
            }
        }
        Ok(matching_rows)
    }

    fn evaluate_expr(&self, expr: &Expr, row: &[Value]) -> Result<Value, ExecutionError> {
        match expr {
            Expr::Identifier(ident) => {
                if ident.quote_style.is_some() {
                    Ok(Value::Varchar(ident.value.clone()))
                } else {
                    let column_name = ident.value.clone();
                    if let Some(column_index) =
                        self.columns.iter().position(|col| col.name == column_name)
                    {
                        return Ok(row[column_index].clone());
                    } else {
                        return Err(ExecutionError::ExecutionError(format!(
                            "列 '{}' 在表 '{}' 中不存在",
                            column_name, self.name
                        )));
                    }
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_value = self.evaluate_expr(left, row)?;
                let right_value = self.evaluate_expr(right, row)?;
                macro_rules! numeric_binop {
                    ($lhs:expr, $rhs:expr, $op:tt) => {
                        match ($lhs, $rhs) {
                            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l $op r)),
                            _ => return Err(ExecutionError::ExecutionError(
                                "不匹配的操作数类型".to_string()
                            ))
                        }
                    }
                }
                macro_rules! relop_binop {
                    ($lhs:expr, $rhs:expr, $op:tt) => {
                        match ($lhs, $rhs) {
                            (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l $op r)),
                            (Value::Varchar(l), Value::Varchar(r)) => Ok(Value::Bool(l $op r)),
                            _ => return Err(ExecutionError::ExecutionError(
                                "不匹配的操作数类型".to_string()
                            ))
                        }
                    };
                }
                macro_rules! bool_binop {
                    ($lhs:expr, $rhs:expr, $op:tt) => {
                        match ($lhs, $rhs) {
                            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Int(if l $op r { 1 } else { 0 })),
                            _ => return Err(ExecutionError::ExecutionError(
                                "不匹配的操作数类型".to_string()
                            ))
                        }
                    }
                }
                match op {
                    BinOp::Plus => numeric_binop!(left_value, right_value, +),
                    BinOp::Minus => numeric_binop!(left_value, right_value, -),
                    BinOp::Multiply => numeric_binop!(left_value, right_value, *),
                    BinOp::Divide => {
                        if let Value::Int(0) = right_value {
                            return Err(ExecutionError::ExecutionError("除数不能为零".to_string()));
                        }
                        numeric_binop!(left_value, right_value, /)
                    }
                    BinOp::Eq => relop_binop!(left_value, right_value, ==),
                    BinOp::NotEq => relop_binop!(left_value, right_value, !=),
                    BinOp::Gt => relop_binop!(left_value, right_value, >),
                    BinOp::Lt => relop_binop!(left_value, right_value, <),
                    BinOp::GtEq => relop_binop!(left_value, right_value, >=),
                    BinOp::LtEq => relop_binop!(left_value, right_value, <=),
                    BinOp::And => bool_binop!(left_value, right_value, &&),
                    BinOp::Or => bool_binop!(left_value, right_value, ||),
                    _ => {
                        return Err(ExecutionError::ExecutionError(format!(
                            "不支持的二元操作符 {}",
                            op.to_string()
                        )))
                    }
                }
            }
            Expr::Value(value) => match &value.value {
                SqlValue::SingleQuotedString(s) => Ok(Value::Varchar(s.clone())),
                SqlValue::Number(n, _) => Ok(Value::Int(n.parse::<i64>().unwrap())),
                SqlValue::Boolean(b) => Ok(Value::Bool(b.clone())),
                SqlValue::Null => Ok(Value::Null),
                _ => Ok(Value::Varchar(value.to_string())),
            },
            _ => {
                return Err(ExecutionError::ExecutionError(format!(
                    "不支持的表达式 {}",
                    expr.to_string()
                )))
            }
        }
    }

    pub fn update_rows(
        &mut self,
        assignments: &Vec<Assignment>,
        where_clause: &Option<Expr>,
    ) -> Result<(), ExecutionError> {
        let matching_row_indices = self.filter_rows(where_clause)?;
        if matching_row_indices.is_empty() {
            return Ok(());
        }
        for row_idx in matching_row_indices {
            for assignment in assignments {
                let column_name = match &assignment.target {
                    AssignmentTarget::ColumnName(name) => name.to_string(),
                    AssignmentTarget::Tuple(_name_vec) => {
                        return Err(ExecutionError::ExecutionError("不支持元组赋值".to_string()));
                    }
                };
                let column_index = self.get_column_index(&column_name);
                if let Some(index) = column_index {
                    let value = self.evaluate_expr(&assignment.value, &self.data[row_idx])?;
                    
                    let mut row = self.data[row_idx].clone();
                    row[index] = value.clone();
                    self.validate_row(&row)?;
                    self.data[row_idx][index] = value;
                    log_info(format!("更新行 {:?} 为 {:?}", row_idx, self.data[row_idx]));
                } else {
                    return Err(ExecutionError::ExecutionError(format!(
                        "列 '{}' 在表 '{}' 中不存在",
                        column_name, self.name
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        self.columns.iter().position(|col| col.name == column_name)
    }

    fn is_primary_key_exists(&self, value: &Value, column: &Column) -> bool {
        if !column.is_primary_key {
            return false;
        }

        if let Some(column_index) = self.columns.iter().position(|col| col.name == column.name) {
            for row in &self.data {
                if row[column_index] == *value {
                    return true;
                }
            }
        }

        false
    }
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

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Varchar(String),
    Bool(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Varchar(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "NULL"),
        }
    }
}

#[derive(Debug)]
pub struct QueryResult {
    /// 列名和类型信息
    pub columns: Vec<Column>,
    /// 实际数据行
    pub rows: Vec<Vec<Value>>,
}

impl QueryResult {
    pub fn new(columns: Vec<Column>, rows: Vec<Vec<Value>>) -> Self {
        Self { columns, rows }
    }

    /// 从一个Table中提取部分列创建QueryResult
    /// 若果指定了列索引，则只提取这些列
    /// 否则提取所有列
    /// # Arguments
    /// * `table` - 要提取的表
    /// * `column_indices` - 可选的列索引，如果为None，则提取所有列
    /// # Returns
    /// * `QueryResult` - 提取的查询结果
    pub fn from_table(table: &Table, column_indices: Option<Vec<usize>>) -> Self {
        let indices = match column_indices {
            Some(idx) => idx,
            None => (0..table.columns.len()).collect(), // 如果没有指定列，则使用所有列
        };

        // 提取列定义
        let result_columns: Vec<Column> =
            indices.iter().map(|&i| table.columns[i].clone()).collect();

        // 提取行数据
        let result_rows: Vec<Vec<Value>> = table
            .data
            .iter()
            .map(|row| indices.iter().map(|&i| row[i].clone()).collect())
            .collect();

        Self::new(result_columns, result_rows)
    }

    /// 打印查询结果表格，格式符合要求
    pub fn display(&self) -> String {
        if self.rows.is_empty() || self.columns.len() == 0 {
            return display_empty_result_message();
        }

        let mut result = String::new();

        // 计算每列的最大宽度
        let mut column_widths: Vec<usize> = vec![0; self.columns.len()];

        // 考虑列标题的宽度
        for (i, col) in self.columns.iter().enumerate() {
            column_widths[i] = col.name.len().max(3); // 至少3个字符宽度
        }

        // 考虑数据行的宽度
        for row in &self.rows {
            for (i, value) in row.iter().enumerate() {
                if i < column_widths.len() {
                    let value_str = match value {
                        Value::Null => String::new(), // NULL值显示为空字符串
                        _ => value.to_string(),
                    };
                    column_widths[i] = column_widths[i].max(value_str.len());
                }
            }
        }

        // 生成表头
        let mut header_line = String::from("|");
        let mut separator_line = String::from("|");

        for (i, col) in self.columns.iter().enumerate() {
            let width = column_widths[i];
            header_line.push_str(&format!(" {:<width$} |", col.name, width = width));
            separator_line.push_str(&format!(" {:<width$} |", "-".repeat(width), width = width));
        }

        result.push_str(&header_line);
        result.push('\n');
        result.push_str(&separator_line);
        result.push('\n');

        // 生成数据行
        for row in &self.rows {
            let mut row_line = String::from("|");

            for (i, value) in row.iter().enumerate() {
                if i < column_widths.len() {
                    let width = column_widths[i];
                    let value_str = match value {
                        Value::Null => String::new(), // NULL值显示为空字符串
                        _ => value.to_string(),
                    };
                    row_line.push_str(&format!(" {:<width$} |", value_str, width = width));
                }
            }

            result.push_str(&row_line);
            result.push('\n');
        }

        result
    }
}

/// 执行查询后如果没有结果，显示特定消息
pub fn display_empty_result_message() -> String {
    "There are no results to be displayed.".to_string()
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
