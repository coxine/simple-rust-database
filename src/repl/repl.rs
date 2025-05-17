/// REPL（读取-求值-打印-循环）实现
///
/// 提供交互式命令行接口，用户可以输入 SQL 语句并查看执行结果。
/// 支持命令历史记录、语法高亮和命令编辑功能。
use crate::executor;
use crate::parser;
use crate::repl::highlighter;
use crate::utils;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Helper;
use rustyline::{error::ReadlineError, Cmd, Editor, KeyEvent, Result};

/// REPL 辅助器
///
/// 为命令行编辑器提供语法高亮、命令验证和其他辅助功能。
struct MyHelper {
    /// SQL 高亮器
    highlighter: highlighter::SqlHighlighter,
}
impl Helper for MyHelper {}

/// 命令验证器实现
///
/// 验证 SQL 输入是否完整，以分号结尾的语句被视为完整语句。
impl Validator for MyHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        use ValidationResult::{Incomplete, Valid};
        let mut input = ctx.input();
        while input.ends_with('\n') || input.ends_with(' ') {
            input = &input[..input.len() - 1];
        }

        let last_line = if let Some(pos) = input.rfind("\n") {
            &input[pos + 1..]
        } else {
            input
        };
        let input = if let Some(pos) = last_line.find("--") {
            &last_line[..pos]
        } else {
            last_line
        };

        let result = if !input.ends_with(';') {
            Incomplete
        } else {
            Valid(None)
        };
        Ok(result)
    }
}

/// 命令补全器缺省接口
impl Completer for MyHelper {
    type Candidate = String;
}

/// 命令提示器缺省接口
impl Hinter for MyHelper {
    type Hint = String;
}

/// 高亮器功能代理到 SqlHighlighter
impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, default)
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: rustyline::highlight::CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

/// 运行 REPL 环境
///
/// 创建一个交互式命令行环境，用户可以在其中输入 SQL 命令并立即看到结果。
/// 支持历史记录、语法高亮和多行输入。
///
/// # Returns
///
/// * `Result<()>` - 执行结果，成功返回 Ok(())，否则返回错误
pub fn run_repl() -> Result<()> {
    println!("欢迎进入 SIMPLE RUST DATABASE Repl，输入 `exit` 或 `Ctrl+D` 退出。");

    let prompt: &str = "> "; // 提示词
    let h = MyHelper {
        highlighter: highlighter::SqlHighlighter::new(prompt),
    };

    let mut rl = Editor::<MyHelper, DefaultHistory>::new()?;
    let history_path = "./data/repl_history.log";
    if std::path::Path::new(history_path).exists() {
        rl.load_history(history_path).unwrap_or_else(|_| {
            println!("无法加载历史记录");
        });
    }

    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::ctrl('j'), Cmd::Insert(1, "\n".to_string())); // Ctrl+J 添加新行

    // 加载数据库表
    match executor::storage::load_all_tables() {
        Ok(_) => {
            utils::log_info("数据加载成功");
        }
        Err(e) => {
            utils::log_error(format!("数据加载失败: {}", e));
        }
    }

    // 主循环
    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                let sql = line.trim();

                // 处理退出命令
                if sql.eq_ignore_ascii_case("exit") {
                    println!("再见！");
                    break;
                }

                // 添加到历史记录
                rl.add_history_entry(sql)?;

                // 解析和执行 SQL
                match parser::parse_sql(sql) {
                    Ok(ast) => {
                        for stmt in &ast {
                            match executor::execute_statement(stmt,sql) {
                                Ok(_) => {}
                                Err(e) => {
                                    utils::log_error(e.to_string());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        utils::log_error(format!(
                            "解析错误: {}\n如需退出请输入 `exit` 或 `Ctrl+D`",
                            e
                        ));
                    }
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }

            Err(ReadlineError::Eof) => {
                println!("\n再见！");
                break;
            }

            Err(err) => {
                utils::log_error(format!("读取错误: {:?}", err));
                break;
            }
        }
    }

    // 保存数据库表
    match executor::storage::store_all_tables() {
        Ok(_) => {
            utils::log_info("数据保存成功");
        }
        Err(e) => {
            utils::log_error(format!("数据保存失败: {}", e));
        }
    }

    // 保存命令历史
    rl.save_history(history_path).unwrap_or_else(|_| {
        println!("无法保存历史记录");
    });
    Ok(())
}
