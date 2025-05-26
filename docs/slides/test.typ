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
#components.adaptive-columns(outline(title: none, indent: 1em))


= 项目简介

== 功能

- 一个用 Rust 编写的简易关系型数据库系统。
#pause
- 基础功能
  - *常用 SQL 支持*：实现了常用的 SQL 语句
  - *数据持久化*：使用 `bincode` 序列化，支持数据在程序重启后的持久保存
#pause
- 特色功能
  - *交互式环境*：提供友好的命令行交互环境
  - *语法高亮*：支持 SQL 关键词、操作符、字符串、注释等的彩色显示
  - *多行输入*：支持多行 SQL 语句，按 `Ctrl+J` 换行
  - *命令历史*：使用上下箭头浏览历史命令

== 项目结构

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

= 解析器

= 执行器

= 特色功能

= 致谢

== 致谢

本项目使用了以下外部库，在此表示感谢：

- #link("https://github.com/apache/datafusion-sqlparser-rs")[`sqlparser`] - 功能强大的 SQL 语句解析器
- #link("https://github.com/kkawakam/rustyline")[`rustyline`] - 提供交互式命令行编辑功能，支持历史记录和语法高亮
- #link("https://github.com/bincode-org/bincode")[`bincode`] - 高效的二进制序列化库，用于数据持久化
- #link("https://github.com/colored-rs/colored")[`colored`] - 终端彩色输出库，提升用户体验
- #link("https://github.com/rust-lang/regex")[`regex`] - 正则表达式库，用于语法高亮和文本处理
- #link("https://github.com/rust-lang-nursery/lazy-static.rs")[`lazy_static`] - 静态变量延迟初始化，优化性能

