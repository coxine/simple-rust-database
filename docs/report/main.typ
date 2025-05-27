#import "template.typ": *

#show: assignment_class.with(
  title: "Rust 数据库大作业 项目报告",
  author: "朱伟鹏 张家浩",
  course: "Rust程序设计语言",
  professor_name: "2025年",
  semester: "春季学期",
  due_time: datetime.today(),
  id: "",
)

= 项目选题

本项目选择实现一个简易的关系型数据库系统，旨在深入理解数据库的基本原理和 Rust 语言的高级特性。通过从零开始构建一个完整的数据库系统，学习 SQL 解析、查询执行、数据存储等核心技术。

= 项目成员及分工

- 朱伟鹏
  + 负责复杂表达式的解析与计算模块。
  + 开发`DROP`、`INSERT`、`UPDATE`、`DELETE`的解析与执行逻辑。
  + 开发交互式命令行环境。
  + 实现语法高亮功能。
- 张家浩
  + 负责项目的整体架构设计与模块划分。
  + 开发 `CREATE` 与 `SELECT` 的解析与执行逻辑。
  + 实现数据持久化功能。
  + 编写项目文档。
  + 负责项目的测试工作。

= 项目需求

== 基本需求

- SQL 语句支持：实现基本数据操作
- 数据类型：支持 `INT`、`VARCHAR` 和 `NULL` 类型
- 约束检查：主键、非空、长度约束
- 数据持久化：程序重启后数据不丢失

== 扩展需求

- 交互式环境：提供友好的命令行界面
- 语法高亮：增强用户体验
- 错误处理：详细的错误信息和异常处理
- 性能优化：使用 `lazy_static` 等技术优化性能

= 整体架构设计

== 项目结构


```plaintext
src/
├── main.rs              # 程序入口
├── lib.rs              # 库接口
├── executor/           # SQL 执行引擎
│   ├── mod.rs         # 执行器模块入口
│   ├── create_table.rs # CREATE TABLE 实现
│   ├── insert.rs      # INSERT 实现
│   ├── query.rs       # SELECT 实现
│   ├── update.rs      # UPDATE 实现
│   ├── delete.rs      # DELETE 实现
│   ├── drop.rs        # DROP TABLE 实现
│   ├── table.rs       # 表结构和操作
│   ├── storage.rs     # 数据持久化
│   └── error.rs       # 错误处理
├── model/              # 数据模型
│   └── mod.rs         # 列、数据类型、值定义
├── parser/             # SQL 解析器
│   ├── mod.rs         # 解析器入口
│   └── error.rs       # 解析错误
├── repl/               # 交互式环境
│   ├── mod.rs         # REPL 模块入口
│   ├── repl.rs        # REPL 实现
│   └── highlighter.rs # 语法高亮
└── utils/              # 工具函数
    ├── mod.rs         # 工具模块入口
    ├── expr_evaluator.rs  # 表达式求值
    └── query_processor.rs # 查询处理
```

== 模块分层

+ 用户接口层：REPL 交互、语法高亮、历史记录
+ 解析器层：生成 AST
+ 执行器层：根据 AST 生成具体的查询
+ 工具函数层：表达式求值、查询处理、结果格式化
+ 数据模型层
  - `Table`：表格
  - `Column`：列定义
  - `Value`：值
+ 存储层：序列化、反序列化

== 设计原则

- 模块化：每个功能独立成模块，便于维护和扩展
- 可扩展：使用 trait 和枚举，便于添加新功能
- 错误处理：完善的错误类型定义和处理机制

= 编译和运行

== 环境要求

- Rust 1.70+
- Cargo

== 编译项目

```bash
cargo build --release
```

== 运行方式

=== 交互式模式

```bash
cargo run
```

=== 文件执行模式

```bash
cargo run -- input.sql
```

= 功能实现

== SQL 支持

=== `CREATE TABLE` - 创建表

- 数据类型支持
  - `INT(length)` - 整数类型，可选长度限制
  - `VARCHAR(length)` - 可变长度字符串，可选长度限制
  - `NULL` - 空值支持
- 约束支持
  - `PRIMARY KEY` - 主键约束，确保唯一性
  - `NOT NULL` - 非空约束
  - 长度约束验证
  - 类型匹配验证


```sql
CREATE TABLE table_name (
    column1 datatype constraints,
    column2 datatype constraints,
);
```

示例：

```sql
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INT(3)
);
```

=== `DROP TABLE` - 删除表

```sql
DROP TABLE table_name [, table_name2, ...];
```

示例：

```sql
DROP TABLE users;
DROP TABLE users, products;  -- 删除多个表
```

=== `INSERT` - 插入数据

```sql
-- 插入所有列的值
INSERT INTO table_name VALUES (value1, value2, ...);

-- 指定列插入
INSERT INTO table_name (column1, column2) VALUES (value1, value2);
```

示例：

```sql
INSERT INTO users VALUES (1, "Alice", 25);
INSERT INTO users (id, name) VALUES (2, "Bob");
```

=== `SELECT` - 查询数据

```sql
-- 基本查询
SELECT * FROM table_name;
SELECT column1, column2 FROM table_name;

-- 带条件查询
SELECT * FROM table_name WHERE condition;

-- 表达式和计算
SELECT name, age * 2 FROM users;
SELECT id, price * 1.1 AS new_price FROM products;
```

支持的条件操作符：

- 比较操作符：`=`、`<`、`>`、`<=`、`>=`、`<>`
- 逻辑操作符：`AND`、`OR`、`NOT`
- 空值检查：`IS NULL`、`IS NOT NULL`
- 数学运算：`+`、`-`、`*`、`/`

=== `UPDATE` - 更新数据

```sql
UPDATE table_name
SET column1 = value1, column2 = value2, ...
WHERE condition;
```

示例：

```sql
UPDATE users SET age = 26 WHERE id = 1;
UPDATE products SET price = price * 1.1 WHERE category = "electronics";
```

=== `DELETE` - 删除数据

```sql
DELETE FROM table_name WHERE condition;
```

示例：

```sql
DELETE FROM users WHERE age < 18;
DELETE FROM products WHERE stock = 0;
```

=== 注释支持

- 单行注释：`-- 这是注释` 或 `# 这是注释`
- 多行注释：`/* 这是多行注释 */`

== 复杂表达式求值

支持在 `WHERE` 条件和 `SELECT` 语句中使用复杂表达式：

```sql
-- 数学运算
SELECT name, age * 2 + 5 AS future_age FROM users;

-- 复杂条件组合
SELECT * FROM products
WHERE (price > 100 AND category = "electronics")
   OR (price < 50 AND stock > 0);

-- 表达式计算
UPDATE products SET price = price * 1.1 WHERE category = "luxury";
```

表达式求值器特性：
- 类型安全：严格的类型检查和转换
- 运算优先级：正确处理数学运算的优先级
- 空值处理：符合 SQL 标准的 `NULL` 值语义
- 错误检测：类型不匹配和运算错误的及时发现

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

= 测试结果

项目包含完整的集成测试套件，位于 `tests/` 目录。

== 运行测试

```bash
# 运行所有测试
cargo test
# 运行特定测试用例
TEST_CASES=11 cargo test
```

== 测试结果

#align(center)[
  #image("../slides/test-result.png")
]


项目通过了所有测试用例，验证了以下功能：

- 基本 CRUD 操作的正确性
- 复杂 `WHERE` 条件查询的准确性
- 表达式计算的精确性
- 数据类型验证的严格性
- 约束检查的完整性
- 错误处理的健壮性
- 多表操作的一致性

= 特色功能

== 交互式命令行环境

项目提供了功能丰富的交互式环境，大大提升了用户体验。

#align(center)[
  #image("../slides/database.png", width: 80%)
]

=== 语法高亮

项目实现了智能的 SQL 语法高亮，支持多种语法元素的彩色显示：
- SQL 关键词：蓝色高亮显示 `SELECT`、`CREATE`、`WHERE` 等
- 操作符：黄色显示比较和数学运算符
- 字符串字面量：绿色显示引号包围的字符串
- 注释：灰色显示单行和多行注释
- 数字：青色显示数值常量
为了避免正则表达式多次加载带来的额外开销，项目中我们使用了 `lazy_static` 将正则表达式作为全局的变量。


```rust
// 高亮规则实现
lazy_static! {
    static ref KEYWORD_RE: Regex = Regex::new(
        r"(?i)\b(SELECT|FROM|WHERE|INSERT|UPDATE|DELETE|CREATE|DROP|TABLE|PRIMARY|KEY|NOT|NULL|AND|OR|VARCHAR|INT)\b"
    ).unwrap();
    static ref OPERATOR_RE: Regex = Regex::new(r"[=<>!]+|[+\-*/]").unwrap();
    static ref STRING_RE: Regex = Regex::new(r#""[^"]*"|'[^']*'"#).unwrap();
    static ref COMMENT_RE: Regex = Regex::new(r"--[^\r\n]*|#[^\r\n]*|/\*.*?\*/").unwrap();
}
```

#align(center)[
  #image("../slides/highlight.png")
]

=== 多行输入

用户在语句内可直接换行，在语句间输入 `Ctrl+J` 可换行，从而便于用户编写复杂的多行 SQL 语句。

```rs
rl.bind_sequence(KeyEvent::ctrl('j'), Cmd::Insert(1, "\n".to_string()));
```

=== 命令历史

项目在本地持久化保存用户每次操作的历史记录，便于未来使用。用户可使用上下箭头浏览历史命令，提升操作效率。

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

== 智能错误处理

应用提供详细准确的错误信息，帮助用户快速定位问题。应用在错误后会中止当前操作，随后可继续使用，不会崩溃退出。



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

pub enum ParserError {
    SqlParseError(String),
}
```

#align(center)[
  #image("../slides/error-display.png")
]

= 遇到的问题

== 内存管理与生命周期

问题：在实现语法高亮时，需要处理字符串的生命周期，确保高亮文本的生命周期与原始文本一致。

解决方案：使用 Rust 的 `Cow` (Clone on Write) 智能指针，避免不必要的字符串复制，提升性能。

```rust
fn highlight<'l>(&self, line: &'l str, pos: usize)
  -> Cow<'l, str>;
fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
      &'s self,prompt: &'p str, default: bool)
  -> Cow<'b, str>;
```

== 表达式求值的复杂性

- 问题：需要支持复杂的 `WHERE` 条件和表达式计算，包括类型转换和运算优先级。
- 解决方案：实现递归的表达式求值器，使用模式匹配处理不同类型的表达式节点，并定义宏简化二元运算符的实现。

```rs
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
```

== 数据持久化性能

- 问题：项目一开始以 CSV 格式作为持久化文件的格式，每次更改后即写入，频繁的文件 I/O 操作可能影响性能。
- 解决方案：使用 `bincode` 库提供的高效的二进制序列化格式，相比文本格式大幅提升序列化/反序列化性能；仅在退出程序时写回数据，提升性能。

```rust
// 代码片段，仅展示写入/读取的主体部分

// 加载表
let table = bincode::decode_from_std_read(&mut file, config::standard())
.map_err(|e|::DeserializationError(file_name.to_string(), e.to_string())})?;

// 保存表
bincode::encode_into_std_write(table, file, config::standard())?;
```

= 项目总结

本项目成功实现了一个功能完整的简易关系型数据库系统，在 Rust 语言学习和数据库系统理解方面都取得了显著成果。

在功能实现方面，项目涵盖了数据库的核心功能，支持包括 `CREATE TABLE`、`DROP TABLE`、`INSERT`、`SELECT`、`UPDATE` 和 `DELETE` 在内的完整 SQL 语句，具备相当高的功能完整性。系统支持 `INT`、`VARCHAR` 和 `NULL` 等基础数据类型，并对字符串长度等参数设置了限制，确保数据类型的正确性和约束性。在数据完整性方面，系统实现了主键约束、非空约束和类型验证等机制，从而保障了数据库中数据的一致性和准确性。此外，数据持久化通过 `bincode` 进行二进制序列化，确保了即使程序重启，数据仍可稳定保存。

为了优化用户体验，项目提供了一个交互式 REPL 命令行环境，支持多行输入与历史记录，极大提升了用户操作的便捷性。我们还利用正则表达式实现了智能语法高亮，能够对 SQL 关键词、操作符、字符串、注释等元素进行彩色显示，使界面更友好、更具可读性。同时，系统配备了详细的错误提示，帮助用户快速定位问题；在性能方面，也通过如 `lazy_static` 等技术提升了语法高亮和数据处理的效率。

在架构设计方面，项目采用了清晰的模块化分层结构，包括用户接口层、解析器层、执行器层、数据模型层和存储层，各层职责分明、耦合度低，便于后续维护与功能扩展。系统中广泛使用了 Rust 的 trait 和枚举机制，增强了代码的灵活性与可拓展性。同时，还建立了一套完善的错误处理系统，确保了程序的健壮性。

通过本项目的实践，我们对 Rust 语言有了更加深入的掌握。通过从零开始构建一个完整的数据库系统，我们不仅深化了对数据库原理的理解，也提升了系统编程的综合能力，为今后进一步深入学习数据库等系统方向的知识奠定了坚实基础。

= 致谢

本项目使用了以下外部库，在此表示感谢：

- #link("https://github.com/apache/datafusion-sqlparser-rs")[`sqlparser`]：功能强大的 SQL 语句解析器
- #link("https://github.com/kkawakam/rustyline")[`rustyline`]：提供交互式命令行编辑功能，支持历史记录和语法高亮
- #link("https://github.com/bincode-org/bincode")[`bincode`]：高效的二进制序列化库，用于数据持久化
- #link("https://github.com/colored-rs/colored")[`colored`]：终端彩色输出库，提升用户体验
- #link("https://github.com/rust-lang/regex")[`regex`]：正则表达式库，用于语法高亮和文本处理
- #link("https://github.com/rust-lang-nursery/lazy-static.rs")[`lazy_static`]：静态变量延迟初始化，优化性能
