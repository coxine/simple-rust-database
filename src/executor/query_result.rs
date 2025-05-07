use sqlparser::ast::{Expr, OrderBy, SelectItem};

use crate::executor::table::Table;
use crate::model::Value;
use crate::utils::query_processor::QueryProcessor;

#[derive(Debug)]
pub struct QueryResult {
    /// 列名
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
        let columns = QueryProcessor::extract_columns_name(table, column_projection)?;
        let sorted_rows = QueryProcessor::sort_rows_by_order(table, order_by_clause)?;
        let rows =
            QueryProcessor::extract_rows(table, &sorted_rows, &row_indices, column_projection)?;
        Ok(Self::new(columns, rows))
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
