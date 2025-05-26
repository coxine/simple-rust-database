# Simple Rust Database

一个用 Rust 编写的简易关系型数据库系统，支持基本的 SQL 操作，包括数据持久化、交互式 REPL 环境和语法高亮功能。

## 🚀 项目特性

### 核心功能

- **常用 SQL 支持**：实现了常用的 SQL 语句
- **数据持久化**：使用 `bincode` 序列化，支持数据在程序重启后的持久保存

### 特色功能

- **交互式环境**：提供友好的命令行交互环境
- **语法高亮**：支持 SQL 关键词、操作符、字符串、注释等的彩色显示
- **多行输入**：支持多行 SQL 语句，按 `Ctrl+J` 换行
- **命令历史**：使用上下箭头浏览历史命令

### 数据类型支持

- `INT(length)` - 整数类型，可选长度限制
- `VARCHAR(length)` - 可变长度字符串，可选长度限制
- `NULL` - 空值支持

### 约束支持

- `PRIMARY KEY` - 主键约束，确保唯一性
- `NOT NULL` - 非空约束
- 长度约束验证
- 类型匹配验证

## 📋 支持的 SQL 语句

### CREATE TABLE - 创建表

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

### DROP TABLE - 删除表

```sql
DROP TABLE table_name [, table_name2, ...];
```

示例：

```sql
DROP TABLE users;
DROP TABLE users, products;  -- 删除多个表
```

### INSERT - 插入数据

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

#### SELECT - 查询数据

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

- 比较操作符：`=`, `<`, `>`, `<=`, `>=`, `<>`
- 逻辑操作符：`AND`, `OR`, `NOT`
- 空值检查：`IS NULL`, `IS NOT NULL`
- 数学运算：`+`, `-`, `*`, `/`

### UPDATE - 更新数据

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

### DELETE - 删除数据

```sql
DELETE FROM table_name WHERE condition;
```

示例：

```sql
DELETE FROM users WHERE age < 18;
DELETE FROM products WHERE stock = 0;
```

### 注释支持

- 单行注释：`-- 这是注释` 或 `# 这是注释`
- 多行注释：`/* 这是多行注释 */`

## 🛠️ 编译和运行

### 环境要求

- Rust 1.70+
- Cargo

### 编译项目

```bash
cargo build --release
```

### 运行方式

#### 交互式模式

```bash
cargo run
```

#### 文件执行模式

```bash
cargo run -- input.sql
```

## 📁 项目结构

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

## 🧪 测试

项目包含完整的集成测试套件，位于 `tests/` 目录：

### 运行所有测试

```bash
cargo test
```

### 运行特定测试用例

```bash
TEST_CASES=11 cargo test
```

### 测试用例覆盖

- 基本 CRUD 操作
- 复杂 `WHERE` 条件查询
- 表达式计算
- 数据类型验证
- 约束检查
- 错误处理
- 多表操作

## 🚧 当前限制

- 不支持 `JOIN` 操作
- 不支持 `GROUP BY` 和聚合函数
- 不支持索引
- 不支持外键约束
- 不支持事务
- 单线程执行

## 🎉 亮点特性

### 高性能语法高亮

- 使用 `lazy_static` 实现正则表达式的懒加载，提升高亮性能
- 智能刷新控制，避免过度渲染

### 灵活的错误处理

- 详细的错误信息，包含具体的错误位置和原因
- 类型不匹配检测
- 约束违反检测
- 主键冲突检测

### 用户友好的 REPL

- 命令历史记录持久化
- 多行输入支持
- 智能语句完成检测
- 优雅的错误提示

## 🌹 致谢

本项目使用了以下外部库，在此表示感谢：

- [`sqlparser`](https://github.com/apache/datafusion-sqlparser-rs) - 功能强大的 SQL 语句解析器
- [`rustyline`](https://github.com/kkawakam/rustyline) - 提供交互式命令行编辑功能，支持历史记录和语法高亮
- [`bincode`](https://github.com/bincode-org/bincode) - 高效的二进制序列化库，用于数据持久化
- [`colored`](https://github.com/colored-rs/colored) - 终端彩色输出库，提升用户体验
- [`regex`](https://github.com/rust-lang/regex) - 正则表达式库，用于语法高亮和文本处理
- [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs) - 静态变量延迟初始化，优化性能
