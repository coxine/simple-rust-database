use lazy_static::lazy_static;
use regex::Regex;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use sqlparser::keywords;
use std::borrow::Cow::{self, Borrowed};
use std::cell::Cell;
use std::time::{Duration, Instant};

// 关键词和正则表达式定义
lazy_static! {
    static ref KEYWORD_RE: Regex = {
        let keywords = [
            "SELECT",
            "FROM",
            "WHERE",
            "INSERT",
            "UPDATE",
            "DELETE",
            "CREATE",
            "DROP",
            "ALTER",
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
    static ref ID_RE: Regex = Regex::new(r"[A-Za-z0-9_]+").unwrap();
    static ref OTHERCHAR_RE: Regex = Regex::new(r"[\u4e00-\u9fa5]+").unwrap();
    static ref WHITESPACE_RE: Regex = Regex::new(r"[\t \n\r]+").unwrap();
    static ref STRING_RE: Regex = Regex::new(r#""(\\.|[^"])*"|'(\\.|[^'])*'"#).unwrap();
    static ref COMMENT_RE: Regex = Regex::new(r"(--[^\n]*)|(\/\*[\s\S]*?\*\/)").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"\b((0[x|X][0-9a-fA-F]+)|(\d+(\.\d+)?))\b").unwrap();
    static ref BRACKET_START_RE: Regex = Regex::new(r"\x1b\[1;34m").unwrap();
    static ref BRACKET_END_RE: Regex = Regex::new(r"\x1b\[0m").unwrap();
}

pub struct SqlHighlighter {
    pub colored_prompt: String,
    pub highlighter: MatchingBracketHighlighter,
    pub last_refresh: Cell<Instant>,
}

impl SqlHighlighter {
    pub fn new(prompt: &str) -> Self {
        SqlHighlighter {
            colored_prompt: format!("\x1b[1;32m{prompt}\x1b[0m").to_owned(),
            highlighter: MatchingBracketHighlighter::new(),
            last_refresh: Cell::new(Instant::now() - Duration::from_millis(30)),
        }
    }
}

impl Highlighter for SqlHighlighter {
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

            // 匹配其他字符 -- 目前仅考虑了中文
            if let Some(m) = OTHERCHAR_RE.find(remaining) {
                if m.start() == 0 {
                    tokens.push(format!("{}", &remaining[m.start()..m.end()]));
                    current_pos += m.end();
                    continue;
                }
            }

            // 如果没有匹配到任何规则，则将当前字符作为普通文本处理

            tokens.push(bracket_str[current_pos..current_pos + 1].to_string());
            current_pos += 1;
        }

        // 拼接所有词素
        let mut ret = tokens.join("");
        while ret.contains("$$Reset ") {
            ret = ret.replace("$$Reset ", "\x1b[0m");
            ret = ret.replace("$$Brack", "\x1b[1;34m");
        }
        Cow::Owned(ret)
    }

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
