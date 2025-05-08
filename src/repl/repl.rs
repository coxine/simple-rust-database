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
use rustyline::{error::ReadlineError, Editor, KeyEvent, Result,Cmd};

struct MyHelper {
    highlighter: highlighter::SqlHighlighter,
}
impl Helper for MyHelper {}

// 验证输入是否以 ; 或 ;--... 结尾，否则继续输入下一行
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
impl Completer for MyHelper {
    type Candidate = String;
}
impl Hinter for MyHelper {
    type Hint = String;
}

// 代理高亮功能到SqlHighlighter
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

pub fn run_repl() -> Result<()> {
    println!("欢迎进入 SIMPLE RUST DATABASE Repl，输入 `exit` 或 `Ctrl+D` 退出。");

    let prompt: &str = "> "; // 提示词
    let h = MyHelper {
        highlighter: highlighter::SqlHighlighter::new(prompt),
    };

    let mut rl = Editor::<MyHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::ctrl('j'), Cmd::Insert(1, "\n".to_string())); // Ctrl+A

    match executor::storage::load_all_tables() {
        Ok(_) => {
            utils::log_info("数据加载成功");
        }
        Err(e) => {
            utils::log_error(format!("数据加载失败: {}", e));
        }
    }
    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                let sql = line.trim();

                if sql.eq_ignore_ascii_case("exit") {
                    println!("再见！");
                    break;
                }

                rl.add_history_entry(sql)?;

                match parser::parse_sql(sql) {
                    Ok(ast) => {
                        for stmt in &ast {
                            match executor::execute_statement(stmt) {
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

    match executor::storage::store_all_tables() {
        Ok(_) => {
            utils::log_info("数据保存成功");
        }
        Err(e) => {
            utils::log_error(format!("数据保存失败: {}", e));
        }
    }
    Ok(())
}
