use sqlparser::ast::{OrderBy, OrderByKind, SelectItem};

use crate::executor::table::Table;
use crate::executor::ExecutionError;
use crate::model::Value;
use crate::utils::expr_evaluator::ExprEvaluator;

pub struct QueryProcessor;

impl QueryProcessor {
    /// 从表数据根据行索引和WHERE条件提取行数据
    /// # Arguments
    /// * `table` - 表数据
    /// * `sorted_indices` - 排序后的行索引
    /// * `filter_indices` - 满足where条件的行索引
    /// * `column_projection` - 列投影
    /// # Returns
    /// * `Result<Vec<Vec<Value>>, ExecutionError>` - 提取的行数据
    pub fn extract_rows(
        table: &Table,
        sorted_indices: &[usize],
        filter_indices: &[usize],
        column_projection: &[SelectItem],
    ) -> Result<Vec<Vec<Value>>, ExecutionError> {
        let mut rows = Vec::new();

        for &idx in sorted_indices {
            // 只处理满足WHERE条件的行
            if !filter_indices.contains(&idx) {
                continue;
            }

            let row = &table.data[idx];
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

    /// 根据ORDER BY子句对表的行进行排序，返回排序后的行索引
    /// # Arguments
    /// * `table` - 要排序的表
    /// * `order_by_clause` - 可选的排序条件
    /// # Returns
    /// * `Result<Vec<usize>, ExecutionError>` - 排序后的行索引
    pub fn sort_rows_by_order(
        table: &Table,
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Vec<usize>, ExecutionError> {
        match order_by_clause {
            Some(order_by) => {
                let order_by_expr = match &order_by.kind {
                    OrderByKind::Expressions(exprs) => exprs,
                    OrderByKind::All(_) => {
                        return Err(ExecutionError::ExecutionError(
                            "不支持的排序类型".to_string(),
                        ));
                    }
                };

                // 创建索引数组
                let mut indices: Vec<usize> = (0..table.data.len()).collect();

                // 根据排序条件对索引进行排序
                indices.sort_by(|&i, &j| {
                    let row1 = &table.data[i];
                    let row2 = &table.data[j];

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

                Ok(indices)
            }
            None => {
                // 没有排序条件时，返回原始索引
                Ok((0..table.data.len()).collect())
            }
        }
    }
}
