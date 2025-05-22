/// 表达式求值模块
///
/// 提供 SQL 表达式求值能力，支持比较操作、算术运算和逻辑运算等。
use sqlparser::ast::{BinaryOperator as BinOp, Expr, Value as SqlValue};

use crate::executor::{table::Table, ExecutionError};
use crate::model::Value;

/// 表达式求值器
///
/// 负责评估 SQL 表达式并返回结果值。
pub struct ExprEvaluator {}

impl ExprEvaluator {
    /// 创建一个新的表达式求值器实例
    pub fn new() -> Self {
        Self {}
    }

    /// 评估表达式并返回结果
    ///
    /// # Arguments
    /// * `table` - 可选的表引用，用于解析列名
    /// * `expr` - 要评估的表达式
    /// * `row` - 可选的当前行数据，用于获取列值
    ///
    /// # Returns
    /// * `Ok(Value)` - 评估结果
    /// * `Err(ExecutionError)` - 评估错误
    ///
    /// # Errors
    /// * `ExecutionError::ExecutionError` - 如果表达式评估失败
    /// * `ExecutionError::TypeUnmatch` - 如果表达式类型不匹配
    pub fn evaluate_expr(
        table: Option<&Table>,
        expr: &Expr,
        row: Option<&[Value]>,
    ) -> Result<Value, ExecutionError> {
        match expr {
            Expr::Identifier(ident) => {
                if ident.quote_style.is_some() {
                    Ok(Value::Varchar(ident.value.clone()))
                } else {
                    let column_name = ident.value.clone();
                    if let (Some(table), Some(row)) = (table, row) {
                        if let Some(column_index) =
                            table.columns.iter().position(|col| col.name == column_name)
                        {
                            return Ok(row[column_index].clone());
                        } else {
                            return Err(ExecutionError::ExecutionError(format!(
                                "列 '{}' 在表 '{}' 中不存在",
                                column_name, table.name
                            )));
                        }
                    } else {
                        return Err(ExecutionError::ExecutionError(
                            "无法在无表环境下解析列标识符".to_string(),
                        ));
                    }
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_value = Self::evaluate_expr(table, left, row)?;
                let right_value = Self::evaluate_expr(table, right, row)?;
                macro_rules! numeric_binop {
                    ($lhs:expr, $rhs:expr, $op:tt) => {
                        match ($lhs, $rhs) {
                            (Value::Null, _) => return Ok(Value::Null),
                            (_, Value::Null) => return Ok(Value::Null),
                            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l $op r)),
                            _ => return Err(ExecutionError::ExecutionError(
                                "不匹配的操作数类型".to_string()
                            ))
                        }
                    }
                }
                macro_rules! relop_binop {
                    ($lhs:expr, $rhs:expr, $op:tt) => {
                        match ($lhs, $rhs) {
                            (Value::Null, _) => return Ok(Value::Null),
                            (_, Value::Null) => return Ok(Value::Null),
                            (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l $op r)),
                            (Value::Varchar(l), Value::Varchar(r)) => Ok(Value::Bool(l $op r)),
                            _ => return Err(ExecutionError::ExecutionError(
                                "不匹配的操作数类型".to_string()
                            ))
                        }
                    };
                }
                macro_rules! bool_binop {
                    ($lhs:expr, $rhs:expr, $op:tt) => {
                        match ($lhs, $rhs) {
                            // 简化版本，含 NULL 则返回 NULL
                            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(if l $op r { true } else { false })),
                            (Value::Null, _) => return Ok(Value::Null),
                            (_, Value::Null) => return Ok(Value::Null),
                            _ => return Err(ExecutionError::ExecutionError(
                                "不匹配的操作数类型".to_string()
                            ))
                        }
                    }
                }
                match op {
                    BinOp::Plus => numeric_binop!(left_value, right_value, +),
                    BinOp::Minus => numeric_binop!(left_value, right_value, -),
                    BinOp::Multiply => numeric_binop!(left_value, right_value, *),
                    BinOp::Divide => {
                        if let Value::Int(0) = right_value {
                            return Err(ExecutionError::ExecutionError("除数不能为零".to_string()));
                        }
                        numeric_binop!(left_value, right_value, /)
                    }
                    BinOp::Eq => relop_binop!(left_value, right_value, ==),
                    BinOp::NotEq => relop_binop!(left_value, right_value, !=),
                    BinOp::Gt => relop_binop!(left_value, right_value, >),
                    BinOp::Lt => relop_binop!(left_value, right_value, <),
                    BinOp::GtEq => relop_binop!(left_value, right_value, >=),
                    BinOp::LtEq => relop_binop!(left_value, right_value, <=),
                    BinOp::And => bool_binop!(left_value, right_value, &&),
                    BinOp::Or => bool_binop!(left_value, right_value, ||),
                    _ => {
                        return Err(ExecutionError::ExecutionError(format!(
                            "不支持的二元操作符 {}",
                            op.to_string()
                        )))
                    }
                }
            }
            Expr::Value(value) => match &value.value {
                SqlValue::SingleQuotedString(s) => Ok(Value::Varchar(s.clone())),
                SqlValue::DoubleQuotedString(s) => Ok(Value::Varchar(s.clone())),
                SqlValue::Number(n, _) => Ok(Value::Int(n.parse::<i64>().unwrap())),
                SqlValue::Boolean(b) => Ok(Value::Bool(b.clone())),
                SqlValue::Null => Ok(Value::Null),
                _ => Ok(Value::Varchar(value.to_string())),
            },
            Expr::IsNull(expr) => {
                let value = Self::evaluate_expr(table, expr, row)?;
                match value {
                    Value::Null => Ok(Value::Bool(true)),
                    _ => Ok(Value::Bool(false)),
                }
            }
            Expr::IsNotNull(expr) => {
                let value = Self::evaluate_expr(table, expr, row)?;
                match value {
                    Value::Null => Ok(Value::Bool(false)),
                    _ => Ok(Value::Bool(true)),
                }
            }
            _ => {
                return Err(ExecutionError::ExecutionError(format!(
                    "不支持的表达式 {}",
                    expr.to_string()
                )))
            }
        }
    }
}

impl Default for ExprEvaluator {
    /// 提供默认构造函数
    fn default() -> Self {
        Self::new()
    }
}
