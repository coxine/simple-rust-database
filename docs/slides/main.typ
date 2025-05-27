#import "touying-theme-nju.typ": *

#show: touying-theme-nju.with(
  config-info(
    title: [Rust 数据库大作业],
    subtitle: [],
    author: [朱伟鹏 张家浩],
    date: datetime.today(),
    institution: [南京大学],
  ),
)
#title-slide()

== Outline <touying:hidden>
#components.adaptive-columns(outline(title: none, indent: 1em,depth: 2))


= 项目概况

== 简介

- 一个用 Rust 编写的简易关系型数据库系统。

#align(center)[
  #image("database.png", width: 80%)
]

== 功能

- 基础功能
  - *常用 SQL 支持*：实现了常用的 SQL 语句
  - *数据持久化*：使用 `bincode` 序列化，支持数据在程序重启后的持久保存
#pause
- 特色功能
  - *交互式环境*：提供友好的命令行交互环境
  - *语法高亮*：支持 SQL 关键词、操作符、字符串、注释等的彩色显示
  - *多行输入*：支持多行 SQL 语句，按 `Ctrl+J` 换行
  - *命令历史*：使用上下箭头浏览历史命令

= 具体实现

== 项目概览

=== 模块化设计

```txt
├── Cargo.lock
├── Cargo.toml
├── data # 持久化数据目录
├── src
│   ├── executor # 执行器模块
│   ├── lib.rs # 库入口
│   ├── main.rs # 程序入口
│   ├── model # 数据类型模块
│   ├── parser # 解析器模块
│   ├── repl # 交互式命令行模块
│   └── utils # 工具函数模块
├── submit.sh # 提交脚本
└── tests # 测试目录
```

=== 程序执行逻辑

+ 程序在`main`中接受至多一个额外参数
  - 无额外参数时，调用`repl`进入交互式命令行
  - 有额外参数时，根据参数打开指定文件，执行其中 SQL 语句
+ `parser`模块解析 SQL 语句
+ `executor`模块执行 SQL 语句


== 解析器 `parser`

- 使用 `sqlparser` 库解析 SQL 语句，将其转换为 AST

```rust
pub fn parse_sql(sql: &str) -> ParserResult<Vec<sqlparser::ast::Statement>> {
    let dialect = MySqlDialect {};
    match Parser::parse_sql(&dialect, sql) {
        Ok(ast) => Ok(ast),
        Err(e) =>
          Err(ParserError::SqlParseError(e.to_string())),
    }
}
```

== 执行器 `executor`

- 模式匹配语法树，根据不同的 SQL 语句类型调用相应的执行函数
- *可拓展性强*

```rust
match stmt {
    Statement::Query(_) => query::query(stmt),
    Statement::CreateTable { .. } => create_table::create_table(stmt),
    Statement::Drop { .. } => drop::drop(stmt),
    Statement::Insert { .. } => insert::insert(stmt),
    Statement::Delete { .. } => delete::delete(stmt),
    Statement::Update { .. } => update::update(stmt),
    _ => Err(ExecutionError::ExecutionError("未识别的命令".to_string())),
}
```

== 表

=== 数据类型 `ColumnDataType`

```rust
pub enum ColumnDataType {
    Int(Option<u64>),      // 整数类型，可选长度限制
    Varchar(Option<u64>),  // 字符串类型，可选长度限制
}
```

=== 数据值 `Value`
```rust
pub enum Value {
    Int(i64),        // 整数值
    Varchar(String), // 字符串值
    Bool(bool),      // 布尔值（预留）
    Null,            // NULL 值
}
```
#pagebreak()

// TODO
实现了该类型的*多态派发*：`Display`, `PartialOrd`, `Clone`
#pause
```rust
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            ...
            // Null is considered less than everything else
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Null, _) => Some(Ordering::Less),
            (_, Value::Null) => Some(Ordering::Greater),
            ...
}
```
#pagebreak()
=== 列 `Column`

```rust
pub struct Column {
    pub name: String,              // 列名
    pub data_type: ColumnDataType, // 列的数据类型
    pub is_primary_key: bool,      // 是否为主键
    pub is_nullable: bool,         // 是否允许 NULL
}
```

支持的约束：
- `PRIMARY KEY`
- `NOT NULL`
- 长度
- 类型匹配


#pagebreak()

=== 表 `Table`
```rust
pub struct Table {
    pub name: String,          // 表名
    pub columns: Vec<Column>,  // 列定义
    pub data: Vec<Vec<Value>>, // 数据行
}
```

- 全局共享数据：

```rust
lazy_static! {
    pub static ref TABLES: Mutex<HashMap<String, Table>>
    // 数据库
    pub static ref EXECUTOR_INPUT: Mutex<String>  
    // 原始 SQL 输入
}
```

=== 方法

```rust
fn validate_row(
  &self,
  values: &[Value]
)  -> Result<(), ExecutionError>; 
// 验证行数据是否合法

pub fn insert_row(
  &mut self,
  values: Vec<Value>
) -> Result<(), ExecutionError>; 
// 插入一行数据


pub fn filter_rows(
  &self,
  where_clause: &Option<Expr>
) -> Result<Vec<usize>, ExecutionError> 
// 根据条件过滤行，返回行索引

pub fn delete_rows(
  &mut self,
  where_clause: &Option<Expr>
) -> Result<(), ExecutionError>; 
// 删除符合条件的行




pub fn update_rows(
    &mut self,
    assignments: &Vec<Assignment>,
    where_clause: &Option<Expr>,
) -> Result<(), ExecutionError>; 
// 更新符合条件的行
```

== 表格层面操作

=== 创建表格 (`CREATE TABLE`)

*实现流程：*
+ 从语法树中解析表名
+ 解析列定义，使用`create_table_columns`函数将其转换为`Vec<Column>`
+ 创建表格，加入全局的`TABLES` 哈希表

```sql
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INT(3)
);
```

#pagebreak()

=== 删除表格 (`DROP TABLE`)

*实现流程：*
+ 从语法树中解析表名
+ 检查表是否存在
+ 从全局的`TABLES` 哈希表中删除表
+ 删除对应的持久化文件

*实现亮点：*
- 支持 `IF EXISTS` 语法

```sql
DROP TABLE users;
DROP TABLE users, products;  -- 支持删除多个表
```

== 数据层面操作

=== 插入数据 (`INSERT INTO`)

*实现流程：*
+ 从语法树中解析表名、字段名、待插入的数据
+ 若仅插入部分列，根据表内实际字段顺序重排数据
+ 调用表格插入方法，验证并插入数据


*实现亮点：*
- 支持部分列插入，自动重排列顺序
- 数据类型验证与约束检查（主键唯一性、非空等）

```sql
INSERT INTO users VALUES (1, "Alice", 25);
INSERT INTO users (id, name) VALUES (2, "Bob");
```

#pagebreak()

=== 表达式处理

- 字面量、关键字求值
- 算术运算：`price * 1.1`, `age + 1`
- 逻辑运算：`age > 18 AND name = "Alice"`
- 空值检查：`name IS NOT NULL`, `name IS NULL`

```rust
match expr {
    Expr::Identifier(ident) => ...
    Expr::BinaryOp { left, op, right } => ...
    Expr::Value(value) => ...
    Expr::IsNull(expr) => ...
    Expr::IsNotNull(expr) => ...
    _ => ...
}
```

#pagebreak()

- *可扩展性*：为二元运算符定义了可复用的宏，未来可便捷地扩展加入位运算、字符串运算等更多表达式类型：
```rust
macro_rules! numeric_binop {
  ($lhs:expr, $rhs:expr, $op:tt) => {
    match ($lhs, $rhs) {
      ...
      (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l $op r)),
      _ => return Err(ExecutionError::ExecutionError(
           "不匹配的操作数类型".to_string()
      ))
    ...
```

#pagebreak()

=== 删除数据 (`DELETE FROM`)

*实现流程：*
+ 从语法树中解析表名、`WHERE`语句
+ 调用表格删除方法，筛选并删除需删除的行

*实现亮点：*
- 从后向前删除，避免索引错位

```sql
DELETE FROM users WHERE age < 18;
```

#pagebreak()


=== 查询数据 (SELECT)

- 实现工具类`QueryProcessor`处理数据查询

```rust
impl QueryProcessor {
    // 提取行数据
    pub fn extract_rows(
        table: &Table,
        sorted_indices: &[usize],
        filter_indices: &[usize],
        column_projection: &[SelectItem],
    ) -> Result<Vec<Vec<Value>>, ExecutionError>;
    // 处理投影
    pub fn process_projection(
        table: Option<&Table>,
        row: Option<&[Value]>,
        column_projection: &[SelectItem],
    ) -> Result<Vec<Value>, ExecutionError>;
    // 提取列名
    pub fn extract_columns_name(
        table: Option<&Table>,
        column_projection: &[SelectItem],
    ) -> Result<Vec<String>, ExecutionError>;
    // 按排序条件排序行
    pub fn sort_rows_by_order(
        table: &Table,
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Vec<usize>, ExecutionError>;
}
```

#pagebreak()

- 实现工具类`QueryResult`展示查询数据

```rust
pub struct QueryResult {
    pub columns: Vec<String>, 
    pub rows: Vec<Vec<Value>>,
}
impl QueryResult {
    pub fn from_table(
        table: Option<&Table>,
        where_clause: &Option<Expr>,
        column_projection: &[SelectItem],
        order_by_clause: &Option<OrderBy>,
    ) -> Result<Self, super::ExecutionError>; // 从表创建结果
    pub fn display(&self) -> String; // 输出结果
}
```

#pagebreak()

*实现流程：*
+ 从语法树中解析表名、列名、查询条件、排序顺序
+ 从表格中过滤出符合查询条件的行
+ 根据顺序进行排序
+ 计算需要查询的结果
+ 转换为`QueryResult`并输出

```sql
-- 基本查询
SELECT * FROM users;
-- 带条件查询
SELECT * FROM users WHERE age > 18;
-- 表达式计算
SELECT name, age * 2 FROM users;
```

=== 更新数据 (`UPDATE`)

*实现流程：*
+ 从语法树中解析表名、`WHERE`语句
+ 从表格中过滤出符合更新条件的行
+ 调用表格更新方法，计算更新后的新值
+ 删除原来的行，验证合法性后并插入表格

```sql
UPDATE users SET age = 26 WHERE id = 1;
UPDATE products SET price = price * 1.1 WHERE category = "electronics";
```

== 持久化存储

- 基于 `bincode` 实现高效的二进制序列化：
  - 程序启动时自动加载所有表
  - 程序退出时自动保存所有表
  - 高效的二进制格式
  - 跨平台兼容性

```rust
let table = 
bincode::decode_from_std_read(&mut file, config::standard())
.map_err(|e|::DeserializationError(file_name.to_string(), e.to_string())})?; // 加载表
  
bincode::encode_into_std_write(table, file, config::standard())?; // 保存表
```

= 项目亮点

== 用户友好的交互

=== 亮点功能

- *语法高亮*：基于`rustyline`，对关键词、数字、运算符、注释等内容进行高亮，提升交互体验。
- *多行输入*：用户可换行输入，查询语句较长时更便利。
- *命令历史*：持久化保存用户每次操作的历史记录，便于未来使用。

#align(center)[
  #image("highlight.png")
]

#pagebreak()

=== 具体实现

```rust
pub fn run_repl() -> Result<()> {
    let prompt = "> ";
    let h = MyHelper { highlighter: SqlHighlighter::new(prompt) };

    let mut rl = Editor::<MyHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::ctrl('j'), Cmd::Insert(1, "\n".to_string()));

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                execute_sql(&line);
            }
            // ... 错误处理
        }
    }
}
```

#pagebreak()

- 使用 `lazy_static` 优化语法高亮中正则表达式的性能

```rust
lazy_static! {
    static ref KEYWORD_RE: Regex = Regex::new(r"(?i)\b(SELECT|FROM|WHERE|...)\b").unwrap();
    static ref OPERATOR_RE: Regex = Regex::new(r"(=|<>|<=|>=|<|>)").unwrap();
    static ref STRING_RE: Regex = Regex::new(r#""(\\.|[^"])*"|'(\\.|[^'])*'"#).unwrap();
    static ref COMMENT_RE: Regex = Regex::new(r"(--[^\n]*)|(\/\*[\s\S]*?\*\/)|(#[^\n]*)").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"\b((0[x|X][0-9a-fA-F]+)|(\d+(\.\d+)?))\b").unwrap();
    ...  
}
```



- 使用Rust特性
  - *多态继承优势*：为高亮器、验证器实现所需 `trait`
  - *生命周期*：高亮文本的生命周期与原始文本一致
  - *智能指针使用*：高亮函数用 `Cow` 来省去具体引用细节
  
```rust
  fn highlight<'l>(&self, line: &'l str, pos: usize) 
    -> Cow<'l, str>;
  fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,prompt: &'p str, default: bool) 
    -> Cow<'b, str>;
```

== 错误处理与验证

=== `executor`

```rust
pub enum ExecutionError {
    TableExists(String),                  // 表已存在错误
    TableNotFound(String),                // 表不存在错误
    TypeUnmatch(String),                  // 类型不匹配错误
    FileError(String),                    // 文件操作错误
    ParseError(String),                   // SQL 语句解析错误
    ExecutionError(String),               // 通用执行错误
    DeserializationError(String, String), // 数据反序列化错误
    SerializationError(String, String),   // 数据序列化错误
    PrimaryKeyConflictError(String),      // 主键冲突错误
}

```

#pagebreak()

=== `parser`

```rust
pub enum ParserError {
    SqlParseError(String),
}
```

=== 错误提示

为所有错误都实现了`Display`，用户交互时可清晰地得知操作错误。

#align(center)[
  #image("error-display.png")
]

== 测试

基于助教老师提供的样例的完整集成测试套件：

```bash
cargo test # 运行所有测试
TEST_CASES = 11 cargo test # 运行特定测试用例
```

测试结果：所有测试用例通过：）

#align(center)[
  #image("test-result.png")
]

== 文档

=== 开发文档

- 每个方法都有详尽的注释，可通过`cargo doc`自动生成文档。

#align(center)[
  #image("cargo-doc.png",width: 80%)
]

=== 用户文档

- `README.md`

#align(center)[
  #image("readme-doc.png",width: 50%)
]

= 致谢

== 致谢

感谢上课风趣幽默的冯洋老师和认真负责的助教们！

本项目使用了以下外部库：

- #link("https://github.com/apache/datafusion-sqlparser-rs")[`sqlparser`] - 功能强大的 SQL 语句解析器
- #link("https://github.com/kkawakam/rustyline")[`rustyline`] - 提供交互式命令行编辑功能，支持历史记录和语法高亮
- #link("https://github.com/bincode-org/bincode")[`bincode`] - 高效的二进制序列化库，用于数据持久化
- #link("https://github.com/colored-rs/colored")[`colored`] - 终端彩色输出库，提升用户体验
- #link("https://github.com/rust-lang/regex")[`regex`] - 正则表达式库，用于语法高亮和文本处理
- #link("https://github.com/rust-lang-nursery/lazy-static.rs")[`lazy_static`] - 静态变量延迟初始化，优化性能

#title-slide()
