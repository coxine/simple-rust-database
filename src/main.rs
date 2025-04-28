mod executor;
mod parser;
mod repl;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl::run_repl()?;
    Ok(())
}
