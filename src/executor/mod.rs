use sqlparser::ast::Statement;
mod create_table;
mod drop;

pub fn execute_statement(stmt: &Statement) {
    match stmt {
        Statement::Query(_) => query(stmt),
        Statement::CreateTable { .. } => create_table::create_csv_table(stmt),
        Statement::Drop { .. } => drop::drop(stmt),
        Statement::Insert { .. } => insert(stmt),
        Statement::Delete { .. } => delete(stmt),
        Statement::Update { .. } => update(stmt),
        _ => eprintln!("未识别的命令"),
    }
}

fn query(stmt: &Statement) {
    println!("Query: {:?}", stmt);
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
