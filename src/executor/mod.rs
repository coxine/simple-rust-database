use sqlparser::ast::Statement;
mod create_table;
mod drop;

pub fn execute_statement(stmt: &Statement) {
    match stmt {
        Statement::Query(_) => query(stmt),
        Statement::CreateTable { .. } => create_table(stmt),
        Statement::Drop { .. } => drop(stmt),
        Statement::Insert { .. } => insert(stmt),
        Statement::Delete { .. } => delete(stmt),
        Statement::Update { .. } => update(stmt),
        _ => eprintln!("未识别的命令"),
    }
}

fn query(stmt: &Statement) {
    println!("Query: {:?}", stmt);
}

fn create_table(stmt: &Statement) {
    if let Statement::CreateTable(create_table_stmt) = stmt {
        create_table::create_csv_table(&create_table_stmt);
    } else {
        eprintln!("创建表失败: 无法解析表名");
    }
}
fn drop(stmt: &Statement) {
    drop::drop(stmt);
}

fn insert(stmt: &Statement) {
    println!("Insert: {:?}", stmt);
}

fn delete(stmt: &Statement) {
    println!("Delete: {:?}", stmt);
}

fn update(stmt: &Statement) {
    println!("Update: {:?}", stmt);
}
