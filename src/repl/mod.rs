mod highlighter;
/// REPL（读取-求值-打印-循环）模块
///
/// 提供交互式命令行界面，允许用户输入 SQL 命令并查看执行结果。
mod repl;
pub use repl::run_repl;
