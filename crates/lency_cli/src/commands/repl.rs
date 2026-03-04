use anyhow::Result;
use std::io::{self, Write};

/// REPL 循环 (实验性)
pub fn cmd_repl() -> Result<()> {
    println!("Lency REPL (Experimental)");
    println!("Type 'exit' or press Ctrl+D to quit.");

    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;
        }

        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        match lency_driver::compile(trimmed) {
            Ok(_res) => {
                println!("Parse OK");
            }
            Err(e) => {
                e.emit(Some("<repl>"), Some(trimmed));
            }
        }
    }

    Ok(())
}
