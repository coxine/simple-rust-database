/// 查询结果模块
///
/// 定义了查询结果的数据结构和格式化方法，用于存储和展示 SQL 查询的结果。
use sqlparser::ast::{Expr, OrderBy, SelectItem};

use crate::executor::table::Table;
use crate::model::Value;
use crate::utils::expr_evaluator::ExprEvaluator;
use crate::utils::query_processor::QueryProcessor;

/// 查询结果结构
///
/// 存储查询返回的列名和数据行，提供格式化输出方法。
#[derive(Debug)]
pub struct QueryResult {
    /// 结果集的列名
    pub columns: Vec<String>,
    /// 结果集的实际数据行
    pub rows: Vec<Vec<Value>>,
}

impl QueryResult {
    /// 创建新的查询结果
    ///
    /// # Arguments
    ///
    /// * `columns` - 结果集的列名列表
    /// * `rows` - 结果集的数据行列表
    ///
    /// # Returns
    ///
    /// 创建的查询结果对象
    pub fn new(columns: Vec<String>, rows: Vec<Vec<Value>>) -> Self {
        Self { columns, rows }
    }

    /// 从表对象创建查询结果
    ///
    /// 根据指定的过滤条件、列投影和排序条件从表中提取数据，构建查询结果。
    /// 如果表为 None，则处理不涉及表的查询（如直接 SELECT 表达式）。
    ///
    /// # Arguments
    ///
    /// * `table` - 可选的表对象，查询的数据源
    /// * `where_clause` - 可选的 WHERE 过滤条件
    /// * `column_projection` - 列投影定义，指定要返回哪些列
    /// * `order_by_clause` - 可选的排序条件
    ///
    /// # Returns
    ///
    /// * `Result<QueryResult, ExecutionError>` - 生成的查询结果或错误
    pub fn from_table(
        table: Option<&Table>,
        where_clause: &Option<Expr>,
        column_projection: &[SelectItem],
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Self, super::ExecutionError> {
        let columns = QueryProcessor::extract_columns_name(table, column_projection)?;
        match table {
            Some(table) => {
                let filter_indices = table.filter_rows(where_clause)?;
                let sorted_indices = QueryProcessor::sort_rows_by_order(table, order_by_clause)?;
                let rows = QueryProcessor::extract_rows(
                    table,
                    &sorted_indices,
                    &filter_indices,
                    column_projection,
                )?;
                Ok(Self::new(columns, rows))
            }
            None => {
                let should_return_row = match where_clause.as_ref() {
                    Some(expr) => matches!(
                        ExprEvaluator::evaluate_expr(None, expr, None),
                        Ok(Value::Bool(true))
                    ),
                    None => true,
                };
                if should_return_row {
                    let rows: Vec<Value> =
                        QueryProcessor::process_projection(None, None, column_projection)?;
                    Ok(Self::new(columns, vec![rows]))
                } else {
                    Ok(Self::new(columns, vec![]))
                }
            }
        }
    }

    /// 格式化查询结果为表格字符串
    ///
    /// 生成包含列名、分隔线和数据行的表格格式输出。
    /// 如果结果为空，返回特定的空结果消息。
    ///
    /// # Returns
    ///
    /// 格式化后的表格字符串
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

/// 无结果消息
///
/// 当查询没有匹配数据时显示的消息。
///
/// # Returns
///
/// 空结果提示消息字符串
pub fn display_empty_result_message() -> String {
    "There are no results to be displayed.".to_string()
}
