use bincode::{Decode, Encode};

use super::ExecutionError;

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
                    if !column.is_nullable {
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
                    return Err(ExecutionError::PrimaryKeyConflictError(format!(
                        "列 '{}' 的值 '{:?}' 已存在",
                        column.name, value
                    )));
                }
            }
        }
        Ok(())
    }

    //     if values.len() != self.columns.len() {
    //         return Err(format!(
    //             "列数不匹配：期望 {}, 实际 {}",
    //             self.columns.len(),
    //             values.len()
    //         ));
    //     }

    //     self.data.push(values);
    //     Ok(())
    // }

    // pub fn delete_rows(&mut self, where_clause: &Option<Expr>) -> Result<usize, String> {
    //     let matching_row_indices = self.filter_rows(where_clause)?;

    //     // 从后向前删除，避免索引错位
    //     let mut matching_row_indices = matching_row_indices;
    //     matching_row_indices.sort_unstable_by(|a, b| b.cmp(a));

    //     for idx in matching_row_indices.iter() {
    //         self.data.remove(*idx);
    //     }

    //     Ok(matching_row_indices.len())
    // }

    // pub fn filter_rows(&self, where_clause: &Option<Expr>) -> Result<Vec<usize>, String> {
    //     if where_clause.is_none() {
    //         // 如果没有 WHERE 子句，返回所有行的索引
    //         return Ok((0..self.data.len()).collect());
    //     }

    //     let expr = where_clause.as_ref().unwrap();
    //     let mut matching_rows = Vec::new();

    //     // 遍历所有行，评估 WHERE 表达式
    //     for (row_idx, row) in self.data.iter().enumerate() {
    //         match self.evaluate_expr(expr, row) {
    //             Ok(true) => matching_rows.push(row_idx),
    //             Ok(false) => {}
    //             Err(e) => return Err(e),
    //         }
    //     }

    //     Ok(matching_rows)
    // }

    // fn evaluate_expr(&self, expr: &Expr, row: &[Value]) -> Result<usize, String> {
    //     // 这里需要实现表达式评估逻辑
    //     // 简单实现示例，实际需要根据您的 Expr 类型结构进行完善
    //     Err("表达式评估尚未实现".to_string())
    // }

    // fn update_rows(&self, assignments: &Vec<Assignment>, where_clause: &Option<Expr>) -> Result<bool, String> {
    //     let matching_row_indices = self.filter_rows(where_clause)?;
    //     if matching_row_indices.is_empty() {
    //         return Ok(0);
    //     }
    //     for row_idx in matching_row_indices {
    //         for assignment in assignments {
    //             let column_name = match assignment.target{
    //                 AssignmentTarget::ColumnName(name) => name.to_string(),
    //                 AssignmentTarget::Tuple(name_vec) => {
    //                     return Err("暂不支持元组赋值".to_string())
    //                 }
    //             }
    //             let column_index = self.get_column_index(column_name);
    //             if let Some(index) = column_index {
    //                 let value = self.evaluate_expr(&assignment.value, &self.data[row_idx])?;
    //                 self.data[row_idx][index] = value;
    //             } else {
    //                 return Err(format!("列 '{}' 不存在", assignment.column_name));
    //             }
    //         }
    //     }
    //     Ok(matching_row_indices.len())
    // }

    // pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
    //     self.columns.iter().position(|col| col.name == column_name)
    // }

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
    Null,
}
