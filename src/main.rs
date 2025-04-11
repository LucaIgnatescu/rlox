use anyhow::Result;
use std::env;

mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jilox [script]");
    } else if args.len() == 2 {
        runFile(&args[1])?;
    } else {
        runPrompt()?;
    }

    Ok(())
}

fn runFile(file_name: &str) -> Result<()> {
    Ok(())
}

fn runPrompt() -> Result<()> {
    Ok(())
}
