use sqlparser::ast::{Expr, OrderBy, OrderByKind, SelectItem};

use crate::executor::table::{Table, Value};
use crate::utils::expr_evaluator::ExprEvaluator;

#[derive(Debug)]
pub struct QueryResult {
    /// 列名和类型信息
    pub columns: Vec<String>,
    /// 实际数据行
    pub rows: Vec<Vec<Value>>,
}

impl QueryResult {
    pub fn new(columns: Vec<String>, rows: Vec<Vec<Value>>) -> Self {
        Self { columns, rows }
    }

    /// 从一个Table中提取部分列创建QueryResult
    /// 若果指定了列索引，则只提取这些列
    /// 否则提取所有列
    /// # Arguments
    /// * `table` - 要提取的表
    /// * `where_clause` - 可选的过滤条件
    /// * `column_projection` - 可选的列投影
    /// * `order_by_clause` - 可选的排序条件
    /// # Returns
    /// * `QueryResult` - 提取的查询结果
    pub fn from_table(
        table: &Table,
        where_clause: &Option<Expr>,
        column_projection: &[SelectItem],
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Self, super::ExecutionError> {
        let row_indices = table.filter_rows(where_clause)?;
        let columns = Self::extract_columns_name(table, column_projection)?;
        let mut rows = Vec::new();
        let sorted_rows = Self::sort_rows_by_order(table, order_by_clause)?;
        for (i, row) in sorted_rows.iter().enumerate() {
            if !row_indices.contains(&i) {
                continue;
            }

            let mut new_row = Vec::new();
            for item in column_projection {
                match item {
                    SelectItem::UnnamedExpr(expr) => {
                        let value = ExprEvaluator::evaluate_expr(table, expr, row)?;
                        new_row.push(value);
                    }
                    SelectItem::Wildcard(_) => {
                        new_row.extend(row.iter().cloned());
                    }
                    _ => {
                        return Err(super::ExecutionError::ExecutionError(format!(
                            "不支持的列投影类型: {}",
                            item
                        )));
                    }
                }
            }
            rows.push(new_row);
        }

        Ok(Self::new(columns, rows))
    }

    /// 根据列投影，从一个表中提取列名
    /// # Arguments
    /// * `table` - 要提取的表
    /// * `column_projection` - 可选的列投影
    /// # Returns
    /// * `Result<Vec<String>, ExecutionError>` - 提取的列
    /// * `ExecutionError` - 执行错误
    fn extract_columns_name(
        table: &Table,
        column_projection: &[SelectItem],
    ) -> Result<Vec<String>, super::ExecutionError> {
        let mut columns = Vec::new();
        for item in column_projection {
            match item {
                SelectItem::UnnamedExpr(expr) => {
                    columns.push(expr.to_string());
                }
                SelectItem::Wildcard(_) => {
                    columns.extend(table.columns.iter().map(|col| col.name.clone()));
                }
                _ => {}
            }
        }
        Ok(columns)
    }

    /// 根据ORDER BY子句对表的行进行排序
    /// # Arguments
    /// * `table` - 要排序的表
    /// * `order_by_clause` - 可选的排序条件
    /// # Returns
    /// * `Result<Vec<Vec<Value>>, ExecutionError>` - 排序后的行
    fn sort_rows_by_order(
        table: &Table,
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Vec<Vec<Value>>, super::ExecutionError> {
        let sorted_rows = match order_by_clause {
            Some(order_by) => {
                let order_by_expr = match &order_by.kind {
                    OrderByKind::Expressions(exprs) => exprs,
                    OrderByKind::All(_) => {
                        return Err(super::ExecutionError::ExecutionError(
                            "不支持的排序类型".to_string(),
                        ));
                    }
                };
                let mut sorted_data = table.data.clone();

                sorted_data.sort_by(|row1, row2| {
                    for order_expr in order_by_expr {
                        let val1 = match ExprEvaluator::evaluate_expr(table, &order_expr.expr, row1)
                        {
                            Ok(val) => val,
                            Err(_) => return std::cmp::Ordering::Equal,
                        };
                        let val2 = match ExprEvaluator::evaluate_expr(table, &order_expr.expr, row2)
                        {
                            Ok(val) => val,
                            Err(_) => return std::cmp::Ordering::Equal,
                        };

                        let comparison =
                            val1.partial_cmp(&val2).unwrap_or(std::cmp::Ordering::Equal);

                        let ordered = if order_expr.options.asc.unwrap_or(true) {
                            comparison // ASC排序 默认
                        } else {
                            comparison.reverse() // DESC排序
                        };

                        if ordered != std::cmp::Ordering::Equal {
                            return ordered;
                        }
                    }
                    std::cmp::Ordering::Equal
                });

                sorted_data
            }
            None => table.data.clone(), // 没有排序条件时，复制原始数据
        };
        Ok(sorted_rows)
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
            column_widths[i] = col.len().max(3); // 至少3个字符宽度
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
            header_line.push_str(&format!(" {:<width$} |", col, width = width));
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
