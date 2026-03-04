use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod lir_backend;
mod path_utils;

use cli::{Cli, Commands};
use commands::{cmd_build, cmd_check, cmd_compile, cmd_repl, cmd_run};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging/verbosity based on flags (Future improvement)
    if cli.verbose {
        // e.g. env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        println!("Verbose mode enabled");
    }

    match cli.command {
        Commands::Compile {
            input,
            output,
            out_dir,
        } => cmd_compile(&input, &output, out_dir.as_deref())?,
        Commands::Run { input, args: _ } => cmd_run(&input)?,
        Commands::Check { input } => cmd_check(&input)?,
        Commands::Build {
            input,
            output,
            out_dir,
            release,
            check_only,
        } => cmd_build(&input, &output, out_dir.as_deref(), release, check_only)?,
        Commands::Repl => cmd_repl()?,
    }

    Ok(())
}
