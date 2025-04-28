mod executor;
mod parser;
mod repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl::run_repl()?;
    Ok(())
}
