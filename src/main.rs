mod executor;
mod highlighter;
mod parser;
mod repl;
fn main() {
    match repl::run_repl() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
