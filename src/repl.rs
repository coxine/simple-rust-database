use crate::executor;
use crate::parser;
use regex::Regex;
use rustyline::completion::Completer;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Helper;
use rustyline::{error::ReadlineError, Editor, Result};
use std::borrow::Cow::{self, Borrowed};

struct MyHelper {
    colored_prompt: String,
    highlighter: MatchingBracketHighlighter,
}
impl Helper for MyHelper {}

// 验证输入是否以 ; 或 ;--... 结尾，否则继续输入下一行
impl Validator for MyHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        use ValidationResult::{Incomplete, Valid};
        let last_line = if let Some(pos) = ctx.input().rfind("\n") {
            &ctx.input()[pos + 1..]
        } else {
            ctx.input()
        };
        let input = if let Some(pos) = last_line.find("--") {
            &ctx.input()[..pos]
        } else {
            ctx.input()
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

// 代码高亮
impl Highlighter for MyHelper {
    // 提示词绿色高亮
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    // 根据光标位置高亮匹配括号
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let keywords = [
            "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP", "ALTER",
            "JOIN", "AND", "OR", "NOT", "GROUP", "ORDER", "BY", "HAVING", "LIMIT", "DISTINCT",
        ];
        let keyword_re = Regex::new(
            &keywords
                .iter()
                .map(|&kw| format!(r"(?i)\b{}\b", regex::escape(kw)))
                .collect::<Vec<String>>()
                .join("|"),
        )
        .unwrap();

        // 数字的正则表达式（包括整数和浮点数）
        let number_re = Regex::new(r"\b(0[x|X][0-9a-fA-F]+)|(\d+(\.\d+)?)\b").unwrap();

        // 对查询字符串进行高亮
        let result = number_re.replace_all(&line, |caps: &regex::Captures| {
            // 对数字进行紫色高亮 先数字防止转义字符被解释为数字
            format!("\x1b[35m{}\x1b[0m", &caps[0])
        });
        let result2 = keyword_re.replace_all(&result, |caps: &regex::Captures| {
            // 对保留字进行蓝色高亮
            format!("\x1b[34m{}\x1b[0m", &caps[0])
        });
        let bracket_str = self.highlighter.highlight(&result2, pos).to_string();
        Cow::Owned(bracket_str)
    }
    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind) || true
    }
}

pub fn run_repl() -> Result<()> {
    println!("欢迎进入 SIMPLE RUST DATABASE Repl，输入 `exit` 或 `Ctrl+D` 退出。");

    let prompt: &str = "> ";
    let h = MyHelper {
        colored_prompt: format!("\x1b[1;32m{prompt}\x1b[0m").to_owned(),
        highlighter: MatchingBracketHighlighter::new(),
    };

    let mut rl = Editor::<MyHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(h));
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
                            executor::execute_statement(stmt);
                        }
                    }
                    Err(e) => {
                        println!("解析错误: {}\n如需退出请输入 `exit` 或 `Ctrl+D`", e);
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
                println!("读取错误: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
