use crate::executor::table::{Column, Table, Value};

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
    /// * `row_indices` - 可选的行索引，如果为None，则提取所有行
    /// * `column_indices` - 可选的列索引，如果为None，则提取所有列
    /// # Returns
    /// * `QueryResult` - 提取的查询结果
    pub fn from_table(
        table: &Table,
        row_indices: Option<Vec<usize>>,
        column_indices: Option<Vec<usize>>,
    ) -> Self {
        let indices = match column_indices {
            Some(idx) => idx,
            None => (0..table.columns.len()).collect(), // 如果没有指定列，则使用所有列
        };

        // 提取列定义
        let result_columns: Vec<Column> =
            indices.iter().map(|&i| table.columns[i].clone()).collect();

        // 提取行数据
        let result_rows: Vec<Vec<Value>> = match row_indices {
            Some(row_idx) => row_idx
                .iter()
                .filter(|&&i| i < table.data.len())
                .map(|&i| {
                    indices
                        .iter()
                        .map(|&col_i| table.data[i][col_i].clone())
                        .collect()
                })
                .collect(),
            None => table
                .data
                .iter()
                .map(|row| indices.iter().map(|&i| row[i].clone()).collect())
                .collect(),
        };

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
