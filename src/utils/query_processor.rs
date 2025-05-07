use sqlparser::ast::{OrderBy, OrderByKind, SelectItem};

use crate::executor::table::Table;
use crate::executor::ExecutionError;
use crate::model::Value;
use crate::utils::expr_evaluator::ExprEvaluator;

pub struct QueryProcessor;

impl QueryProcessor {
    /// 从排序后的行和行索引中提取满足条件的行数据
    /// # Arguments
    /// * `table` - 表数据
    /// * `sorted_rows` - 排序后的行
    /// * `row_indices` - 满足where条件的行索引
    /// * `column_projection` - 列投影
    /// # Returns
    /// * `Result<Vec<Vec<Value>>, ExecutionError>` - 提取的行数据
    pub fn extract_rows(
        table: &Table,
        sorted_rows: &[Vec<Value>],
        row_indices: &[usize],
        column_projection: &[SelectItem],
    ) -> Result<Vec<Vec<Value>>, ExecutionError> {
        let mut rows = Vec::new();

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
                        return Err(ExecutionError::ExecutionError(format!(
                            "不支持的列投影类型: {}",
                            item
                        )));
                    }
                }
            }
            rows.push(new_row);
        }

        Ok(rows)
    }

    /// 根据列投影，从一个表中提取列名
    /// # Arguments
    /// * `table` - 要提取的表
    /// * `column_projection` - 可选的列投影
    /// # Returns
    /// * `Result<Vec<String>, ExecutionError>` - 提取的列
    /// * `ExecutionError` - 执行错误
    pub fn extract_columns_name(
        table: &Table,
        column_projection: &[SelectItem],
    ) -> Result<Vec<String>, ExecutionError> {
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
    pub fn sort_rows_by_order(
        table: &Table,
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Vec<Vec<Value>>, ExecutionError> {
        let sorted_rows = match order_by_clause {
            Some(order_by) => {
                let order_by_expr = match &order_by.kind {
                    OrderByKind::Expressions(exprs) => exprs,
                    OrderByKind::All(_) => {
                        return Err(ExecutionError::ExecutionError(
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
}
