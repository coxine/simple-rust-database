use bincode::{Decode, Encode};

use crate::model::{Column, ColumnDataType, Value};
use crate::utils::expr_evaluator::ExprEvaluator;
use crate::utils::log_info;

use super::ExecutionError;
use sqlparser::ast::{Assignment, AssignmentTarget, Expr};

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
            match ExprEvaluator::evaluate_expr(Some(self), expr, Some(row)) {
                Ok(Value::Bool(true)) => matching_rows.push(row_idx),
                Ok(Value::Bool(false)) => {}
                Ok(Value::Null) => {}
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
                    let value = ExprEvaluator::evaluate_expr(
                        Some(self),
                        &assignment.value,
                        Some(&self.data[row_idx]),
                    )?;

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
