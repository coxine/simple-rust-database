use crate::executor::{ExecutionError, ExecutionResult};
use sqlparser::ast::{FromTable, Statement, TableFactor};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn delete(stmt: &Statement) -> ExecutionResult<()> {
    if let Statement::Delete(delete) = stmt {
        let tables = match &delete.from {
            FromTable::WithFromKeyword(vec) => vec,
            FromTable::WithoutKeyword(vec) => vec,
        };
        for table in tables {
            let table_name = match &table.relation {
                TableFactor::Table { name, .. } => name,
                _ => {
                    return Err(ExecutionError::ExecutionError(
                        "暂时无法解析的表名".to_string(),
                    ));
                }
            };
            let file_path = format!("data/{}.csv", table_name);
            // 打开文件并读取内容
            let file = File::open(&file_path).map_err(|_| {
                ExecutionError::ExecutionError(format!("无法打开文件: {}", file_path))
            })?;
            let reader = BufReader::new(file);

            // 读取表头、长度行和标志行
            let mut lines = reader.lines();

            let header_line = lines
                .next()
                .ok_or_else(|| ExecutionError::ExecutionError("文件缺少表头信息".to_string()))?;
            let headers: Vec<String> = header_line
                .as_ref()
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect();

            let length_line = lines
                .next()
                .ok_or_else(|| ExecutionError::ExecutionError("文件缺少长度信息".to_string()))?;
            let lengths: Vec<Option<u32>> = length_line
                .as_ref()
                .unwrap()
                .split(',')
                .map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        s.parse::<u32>().ok()
                    }
                })
                .collect();

            let flag_line = lines
                .next()
                .ok_or_else(|| ExecutionError::ExecutionError("文件缺少标志信息".to_string()))?;
            let flags: Vec<u8> = flag_line
                .as_ref()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u8>().unwrap_or(0))
                .collect();

            // 解析 WHERE 子句
            let where_clause = delete.selection.as_ref();

            // 筛选需要保留的行
            let mut remaining_rows = Vec::new();
            remaining_rows.push(header_line.unwrap());
            remaining_rows.push(length_line.unwrap());
            remaining_rows.push(flag_line.unwrap());

            for line in lines {
                let line = line
                    .map_err(|_| ExecutionError::ExecutionError("读取文件行时出错".to_string()))?;

                let _columns: Vec<&str> = line.split(',').collect();

                // 如果没有 WHERE 子句，删除所有行
                if !where_clause.is_none()
                /*|| match_where(where_clause.unwrap(), &columns,&headers,&lengths,&flags)*/
                {
                    remaining_rows.push(line);
                } else {
                    println!("Delete line: {}", line);
                }
            }

            // 将剩余的行写回文件
            let mut file = File::create(&file_path).map_err(|_| {
                ExecutionError::ExecutionError(format!("无法写入文件: {}", file_path))
            })?;
            for row in remaining_rows {
                writeln!(file, "{}", row)
                    .map_err(|_| ExecutionError::ExecutionError("写入文件时出错".to_string()))?;
            }
        }
    } else {
        return Err(ExecutionError::ParseError("无法解析DELETE语句".to_string()));
    }
    Ok(())
}
