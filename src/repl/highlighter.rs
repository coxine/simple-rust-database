/// SQL 语法高亮模块
///
/// 提供 SQL 语法高亮功能，增强用户在 REPL 环境中的交互体验。
/// 支持 SQL 关键词、操作符、字符串、注释等元素的彩色显示。
use lazy_static::lazy_static;
use regex::Regex;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use std::borrow::Cow::{self, Borrowed};
use std::cell::Cell;
use std::time::{Duration, Instant};

lazy_static! {
    /// SQL 关键词的正则表达式
    static ref KEYWORD_RE: Regex = {
        let keywords = [
            "ASC",
            "DESC",
            "SELECT",
            "FROM",
            "WHERE",
            "INSERT",
            "UPDATE",
            "DELETE",
            "CREATE",
            "DROP",
            "ALTER",
            "INTO",
            "VALUES",
            "SET",
            "TABLE",
            "JOIN",
            "GROUP",
            "ORDER",
            "BY",
            "HAVING",
            "LIMIT",
            "DISTINCT",
            "NULL",
            "PRIMARY",
            "KEY",
            "FOREIGN",
            "REFERENCES",
            "UNIQUE",
            "CHECK",
            "DEFAULT",
            "INDEX",
            "VIEW",
        ];
        Regex::new(&format!(
            "(?i){}",
            &keywords
                .iter()
                .map(|&kw| format!(r"\b{}\b", regex::escape(kw)))
                .collect::<Vec<String>>()
                .join("|"),
        ))
        .unwrap()
    };

    /// SQL 操作符的正则表达式
    static ref OPERATOR_RE: Regex = {
        let operator = [
            "AND",
            "OR",
            "NOT",
            "COUNT",
            "SUM",
            "AVG",
            "MAX",
            "MIN",
            "LIKE",
            "IN",
            "BETWEEN",
            "IS",
            "EXISTS",
            "AS",
            "ON",
            "WITH",
            "UNION",
            "INTERSECT",
        ];
        Regex::new(&format!(
            "(?i){}",
            operator
                .iter()
                .map(|&kw| format!(r"\b{}\b", kw))
                .collect::<Vec<String>>()
                .join("|")
        ))
        .unwrap()
    };

    /// 标识符的正则表达式
    static ref ID_RE: Regex = Regex::new(r"[A-Za-z0-9_]+").unwrap();

    /// 其它字符的正则表达式
    static ref OTHERCHAR_RE: Regex = Regex::new(r".").unwrap();

    /// 空白字符的正则表达式
    static ref WHITESPACE_RE: Regex = Regex::new(r"[\t \n\r]+").unwrap();

    /// 字符串字面量的正则表达式
    static ref STRING_RE: Regex = Regex::new(r#""(\\.|[^"])*"|'(\\.|[^'])*'"#).unwrap();

    /// 注释的正则表达式
    static ref COMMENT_RE: Regex = Regex::new(r"(--[^\n]*)|(\/\*[\s\S]*?\*\/)|(#[^\n]*)").unwrap();

    /// 数字的正则表达式
    static ref NUMBER_RE: Regex = Regex::new(r"\b((0[x|X][0-9a-fA-F]+)|(\d+(\.\d+)?))\b").unwrap();

    /// 括号高亮起始标记的正则表达式
    static ref BRACKET_START_RE: Regex = Regex::new(r"\x1b\[1;34m").unwrap();

    /// 括号高亮结束标记的正则表达式
    static ref BRACKET_END_RE: Regex = Regex::new(r"\x1b\[0m").unwrap();
}

/// SQL 高亮器
///
/// 为 REPL 环境提供 SQL 语法高亮功能。
pub struct SqlHighlighter {
    /// 着色后的提示符
    pub colored_prompt: String,
    /// 括号匹配高亮器
    pub highlighter: MatchingBracketHighlighter,
    /// 上次刷新时间戳，用于控制高亮频率
    pub last_refresh: Cell<Instant>,
}

impl SqlHighlighter {
    /// 创建新的 SQL 高亮器实例
    ///
    /// # Arguments
    ///
    /// * `prompt` - 命令提示符字符串
    ///
    /// # Returns
    ///
    /// 创建好的 SqlHighlighter 实例
    pub fn new(prompt: &str) -> Self {
        SqlHighlighter {
            colored_prompt: format!("\x1b[1;32m{prompt}\x1b[0m").to_owned(),
            highlighter: MatchingBracketHighlighter::new(),
            last_refresh: Cell::new(Instant::now() - Duration::from_millis(30)),
        }
    }
}

impl Highlighter for SqlHighlighter {
    /// 高亮提示符
    ///
    /// 将提示符以绿色显示。
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

    /// 高亮输入行
    ///
    /// 对 SQL 查询字符串进行语法高亮，以不同颜色显示 SQL 的各个部分。
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        // 对查询字符串进行高亮
        // 根据光标位置高亮匹配括号
        let mut bracket_str = self.highlighter.highlight(&line, pos).to_string();
        bracket_str = BRACKET_START_RE
            .replace_all(&bracket_str, "$$$$Brack")
            .to_string();
        bracket_str = BRACKET_END_RE
            .replace_all(&bracket_str, "$$$$Reset ")
            .to_string();
        let mut tokens = Vec::new();
        let mut current_pos = 0;

        while current_pos < bracket_str.len() {
            let remaining = &&bracket_str[current_pos..];

            // 匹配空白符
            if let Some(m) = WHITESPACE_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("{}", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 匹配字符串字面量
            if let Some(m) = STRING_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("\x1b[32m{}\x1b[0m", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 匹配注释
            if let Some(m) = COMMENT_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("\x1b[90m{}\x1b[0m", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 匹配数字
            if let Some(m) = NUMBER_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("\x1b[35m{}\x1b[0m", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 匹配关键词
            if let Some(m) = KEYWORD_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("\x1b[34m{}\x1b[0m", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            if let Some(m) = OPERATOR_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("\x1b[36m{}\x1b[0m", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 匹配单词(Identifier)
            if let Some(m) = ID_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("{}", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 如果没有匹配到任何规则，则将当前字符作为普通文本处理
            // 匹配其他字符
            if let Some(m) = OTHERCHAR_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("{}", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }
        }

        // 拼接所有词素
        let mut ret = tokens.join("");
        while ret.contains("$$Reset ") {
            ret = ret.replace("$$Reset ", "\x1b[0m");
            ret = ret.replace("$$Brack", "\x1b[1;34m");
        }
        Cow::Owned(ret)
    }

    /// 高亮字符
    ///
    /// 控制刷新频率，避免过于频繁的刷新降低性能。
    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        let now = Instant::now();
        let last = self.last_refresh.get();
        let mut flag = false;
        if now.duration_since(last) >= Duration::from_millis(50) {
            self.last_refresh.set(now);
            flag = true;
        }
        self.highlighter.highlight_char(line, pos, kind) || flag
    }
}
