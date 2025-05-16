/// 数据库表模块
///
/// 定义了表的数据结构和操作方法，包括行的插入、删除、更新和过滤等。
use bincode::{Decode, Encode};

use crate::model::{Column, ColumnDataType, Value};
use crate::utils::expr_evaluator::ExprEvaluator;
use crate::utils::log_info;

use super::ExecutionError;
use sqlparser::ast::{Assignment, AssignmentTarget, Expr};

/// 表结构
///
/// 存储表的元数据（表名和列定义）以及实际的行数据。
/// 支持序列化和反序列化以实现持久化存储。
#[derive(Debug, Encode, Decode)]
pub struct Table {
    /// 表名
    pub name: String,
    /// 表的列定义
    pub columns: Vec<Column>,
    /// 表中的数据行
    pub data: Vec<Vec<Value>>,
}

impl Table {
    /// 创建新表
    ///
    /// # Arguments
    ///
    /// * `name` - 表名
    /// * `columns` - 列定义列表
    ///
    /// # Returns
    ///
    /// 创建的表对象
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self {
            name,
            columns,
            data: Vec::new(),
        }
    }

    /// 插入一行数据
    ///
    /// 在插入前会验证数据是否符合表的约束条件。
    ///
    /// # Arguments
    ///
    /// * `values` - 要插入的值列表，顺序需与表的列定义一致
    ///
    /// # Returns
    ///
    /// * `Ok(())` 插入成功
    /// * `Err(ExecutionError)` 插入失败，包含详细错误信息
    pub fn insert_row(&mut self, values: Vec<Value>) -> Result<(), ExecutionError> {
        self.validate_row(&values)?;
        self.data.push(values);
        Ok(())
    }

    /// 验证行数据是否符合表的约束
    ///
    /// 进行的验证包括：
    /// - 检查值的数量是否与列数匹配
    /// - 验证每个值的类型是否与对应列的类型匹配
    /// - 确保整数和字符串值不超过其定义的长度限制
    /// - 防止在非空或主键列中插入 NULL 值
    /// - 确保主键不重复
    ///
    /// # Arguments
    ///
    /// * `values` - 表示要插入的一行的值切片
    ///
    /// # Returns
    ///
    /// * `Ok(())` 如果所有验证都通过
    /// * `Err(ExecutionError)` 如果任何验证失败，包含详细错误信息
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
                        println!(
                            "Error: Field '{}' doesn't have a default value",
                            column.name
                        );
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

    /// 删除满足条件的行
    ///
    /// # Arguments
    ///
    /// * `where_clause` - 可选的 WHERE 条件表达式，用于过滤要删除的行
    ///
    /// # Returns
    ///
    /// * `Ok(())` 删除成功
    /// * `Err(ExecutionError)` 删除失败
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

    /// 过滤满足条件的行
    ///
    /// 根据可选的 WHERE 条件表达式筛选出满足条件的行索引。
    ///
    /// # Arguments
    ///
    /// * `where_clause` - 可选的 WHERE 条件表达式
    ///
    /// # Returns
    ///
    /// * `Result<Vec<usize>, ExecutionError>` - 满足条件的行索引列表
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
                        "筛选条件必须是可判断的表达式".to_string(),
                    ))
                }
                Err(e) => return Err(e),
            }
        }
        Ok(matching_rows)
    }

    /// 更新满足条件的行
    ///
    /// # Arguments
    ///
    /// * `assignments` - 列赋值表达式列表
    /// * `where_clause` - 可选的 WHERE 条件表达式
    ///
    /// # Returns
    ///
    /// * `Result<(), ExecutionError>` - 更新成功或失败
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
            let mut row = self.data[row_idx].clone();
            let original_row = row.clone();
            self.data.remove(row_idx);
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
                        Some(&original_row),
                    )?;
                    row[index] = value.clone();
                } else {
                    return Err(ExecutionError::ExecutionError(format!(
                        "列 '{}' 在表 '{}' 中不存在",
                        column_name, self.name
                    )));
                }
            }
            match self.validate_row(&row) {
                Ok(_) => self.data.insert(row_idx, row),
                Err(e) => {
                    self.data.insert(row_idx, original_row);
                    return Err(e);
                }
            };
            log_info(format!("更新行 {:?} 为 {:?}", row_idx, self.data[row_idx]));
        }
        Ok(())
    }

    /// 获取列索引
    ///
    /// 根据列名查找其在表中的索引位置。
    ///
    /// # Arguments
    ///
    /// * `column_name` - 列名
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - 如果存在该列，返回其索引；否则返回 None
    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        self.columns.iter().position(|col| col.name == column_name)
    }

    /// 检查主键值是否已存在
    ///
    /// # Arguments
    ///
    /// * `value` - 要检查的值
    /// * `column` - 列定义
    ///
    /// # Returns
    ///
    /// * `bool` - 如果主键值已存在，返回 true；否则返回 false
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
