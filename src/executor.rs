use sqlparser::ast::Statement;

pub fn execute_statement(stmt: &Statement) {
    match stmt {
        Statement::Query(_) => query(stmt),
        Statement::CreateTable { .. } => create_table(stmt),
        Statement::Drop { .. } => drop(stmt),
        Statement::Insert { .. } => insert(stmt),
        Statement::Delete { .. } => delete(stmt),
        Statement::Update { .. } => update(stmt),
        _ => println!("未识别的命令"),
    }
}

fn query(stmt: &Statement) {
    println!("Query: {:?}", stmt);
}
fn create_table(stmt: &Statement) {
    println!("CreateTable: {:?}", stmt);
}
fn drop(stmt: &Statement) {
    println!("Drop: {:?}", stmt);
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
