use crate::executor::table::Table;
use crate::executor::ExecutionError;
use crate::executor::EXECUTOR_INPUT;
use crate::model::Value;
use crate::utils::expr_evaluator::ExprEvaluator;
/// 查询处理器模块
///
/// 提供查询处理功能，包括处理查询投影、过滤和排序等操作。
use sqlparser::ast::{OrderBy, OrderByKind, SelectItem, Spanned};
use sqlparser::tokenizer::Location;

fn extract_original_str(s: &str, start: Location, end: Location) -> Option<String> {
    let lines: Vec<&str> = s.lines().collect();

    let start_line = (start.line - 1) as usize;
    let end_line = (end.line - 1) as usize;

    if start_line!=end_line{
        return None;
    }

    let ret = lines[start_line]
        .chars()
        .skip(start.column as usize - 1)
        .take((end.column- start.column) as usize)
        .collect::<String>();
    Some(ret)
}

/// 查询处理器
///
/// 提供对查询结果的处理方法，包括列提取、行过滤和排序等。
pub struct QueryProcessor;

impl QueryProcessor {
    /// 提取行数据
    ///
    /// 根据过滤条件和列投影提取行数据
    ///
    /// # Arguments
    ///
    /// * `table` - 表对象
    /// * `sorted_indices` - 排序后的行索引
    /// * `filter_indices` - 过滤后的行索引
    /// * `column_projection` - 列投影列表
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Vec<Value>>, ExecutionError>` - 结果行数据或错误
    pub fn extract_rows(
        table: &Table,
        sorted_indices: &[usize],
        filter_indices: &[usize],
        column_projection: &[SelectItem],
    ) -> Result<Vec<Vec<Value>>, ExecutionError> {
        sorted_indices
            .iter()
            .filter(|&&idx| filter_indices.contains(&idx))
            .map(|&idx| {
                let row = &table.data[idx];
                let values = Self::process_projection(Some(table), Some(row), column_projection)?;
                Ok(values)
            })
            .collect()
    }

    /// 处理投影
    ///
    /// 根据列投影处理一行数据
    ///
    /// # Arguments
    ///
    /// * `table` - 可选的表对象
    /// * `row` - 可选的行数据
    /// * `column_projection` - 列投影列表
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Value>, ExecutionError>` - 处理后的行数据或错误
    pub fn process_projection(
        table: Option<&Table>,
        row: Option<&[Value]>,
        column_projection: &[SelectItem],
    ) -> Result<Vec<Value>, ExecutionError> {
        let values = column_projection
            .iter()
            .map(|item| match item {
                SelectItem::UnnamedExpr(expr) => {
                    ExprEvaluator::evaluate_expr(table, expr, row).map(|val| vec![val])
                }
                SelectItem::Wildcard(_) => Ok(row.unwrap().to_vec()),
                _ => Err(ExecutionError::ExecutionError(format!(
                    "不支持的列投影类型: {}",
                    item
                ))),
            })
            .collect::<Result<Vec<Vec<Value>>, ExecutionError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<Value>>();
        Ok(values)
    }

    /// 提取列名
    ///
    /// 从查询的选择项中提取列名
    ///
    /// # Arguments
    ///
    /// * `table` - 可选的表对象
    /// * `column_projection` - 列投影列表
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>, ExecutionError>` - 列名列表或错误
    pub fn extract_columns_name(
        table: Option<&Table>,
        column_projection: &[SelectItem],
    ) -> Result<Vec<String>, ExecutionError> {
        Ok(column_projection
            .iter()
            .flat_map(|item| match item {
                SelectItem::UnnamedExpr(expr) => vec![extract_original_str(
                    &EXECUTOR_INPUT.lock().unwrap().to_string(),
                    expr.span().start,
                    expr.span().end,
                )
                .unwrap()],
                SelectItem::Wildcard(_) => table
                    .unwrap()
                    .columns
                    .iter()
                    .map(|col| col.name.clone())
                    .collect(),
                _ => vec![],
            })
            .collect())
    }

    /// 按排序条件排序行
    ///
    /// # Arguments
    ///
    /// * `table` - 表对象
    /// * `order_by` - 可选的排序条件
    ///
    /// # Returns
    ///
    /// * `Result<Vec<usize>, ExecutionError>` - 排序后的行索引或错误
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
                        let val1 = match ExprEvaluator::evaluate_expr(
                            Some(table),
                            &order_expr.expr,
                            Some(row1),
                        ) {
                            Ok(val) => val,
                            Err(_) => return std::cmp::Ordering::Equal,
                        };
                        let val2 = match ExprEvaluator::evaluate_expr(
                            Some(table),
                            &order_expr.expr,
                            Some(row2),
                        ) {
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
