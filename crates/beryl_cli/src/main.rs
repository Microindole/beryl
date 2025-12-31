use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "beryl")]
#[command(about = "The Beryl Programming Language Compiler", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check syntax and types without generating code
    Check {
        /// Input file path (.brl)
        path: String,
    },
    
    /// Compile source file to executable
    Build {
        /// Input file path (.brl)
        path: String,
        
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Run source file directly (JIT or Compile-and-Run)
    Run {
        /// Input file path (.brl)
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { path } => {
            println!("🔍 Checking: {}", path);
            // TODO: 调用 beryl_driver::check(path)
        },
        Commands::Build { path, output } => {
            println!("🔨 Building: {} -> {:?}", path, output);
            // TODO: 调用 beryl_driver::build(path, output)
        },
        Commands::Run { path } => {
            println!("🚀 Running: {}", path);
            // TODO: 调用 beryl_driver::run(path)
        }
    }
}