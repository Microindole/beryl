use anyhow::Result;
use clap::{Parser, Subcommand};
use lency_driver::compile_file;
use std::fs;

#[derive(Parser)]
#[command(name = "lencyc")]
#[command(
    about = "Lency 编译器 - 简洁、规范、清晰",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// 详细输出模式
    #[arg(short, long, global = true)]
    verbose: bool,

    /// 安静模式 (只输出错误)
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 编译 Lency 源文件为 LLVM IR
    Compile {
        /// 输入文件
        input: String,

        /// 输出文件 (默认: lencyTemp.ll)
        #[arg(short, long, default_value = "lencyTemp.ll")]
        output: String,
    },

    /// 编译并运行 Lency 程序
    Run {
        /// 输入文件
        input: String,

        /// 传递给程序的参数
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// 检查语法和语义错误
    Check {
        /// 输入文件
        input: String,
    },

    /// 编译并生成可执行文件
    Build {
        /// 输入文件
        input: String,

        /// 输出文件 (默认: lencyTemp.out)
        #[arg(short, long, default_value = "lencyTemp.out")]
        output: String,

        /// 优化构建 (Release mode)
        #[arg(long)]
        release: bool,
    },

    /// 交互式 REPL (实验性)
    Repl,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging/verbosity based on flags (Future improvement)
    if cli.verbose {
        // e.g. env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        println!("Verbose mode enabled");
    }

    match cli.command {
        Commands::Compile { input, output } => cmd_compile(&input, &output)?,
        Commands::Run { input, args: _ } => cmd_run(&input)?,
        Commands::Check { input } => cmd_check(&input)?,
        Commands::Build {
            input,
            output,
            release,
        } => cmd_build(&input, &output, release)?,
        Commands::Repl => cmd_repl()?,
    }

    Ok(())
}

/// 编译命令
fn cmd_compile(input: &str, output: &str) -> Result<()> {
    println!("Compiling {} ...", input);

    let source = fs::read_to_string(input)?;
    let result = match lency_driver::compile(&source) {
        Ok(res) => res,
        Err(e) => {
            e.emit(Some(input), Some(&source));
            std::process::exit(1);
        }
    };

    fs::write(output, result.ir)?;
    println!("Generated {}", output);

    Ok(())
}

/// 运行命令
fn cmd_run(input: &str) -> Result<()> {
    println!("Running {} ...", input);

    // 1. 编译
    let result = compile_file(input)?;

    // 2. 写临时文件
    let temp_ir = "/tmp/lency_temp.ll";
    fs::write(temp_ir, result.ir)?;

    // 3. 使用 lli 运行 LLVM IR
    let mut cmd = std::process::Command::new("lli-15");

    // 加载运行时库
    // 尝试在 target/debug 和 target/release 中查找
    let mut runtime_found = false;
    if let Ok(cwd) = std::env::current_dir() {
        // Check for .so (Linux) or .dylib (macOS)
        let libs = ["liblency_runtime.so", "liblency_runtime.dylib"];
        let dirs = ["target/debug", "target/release"];

        for dir in dirs {
            for lib in libs {
                let lib_path = cwd.join(dir).join(lib);
                if lib_path.exists() {
                    cmd.arg(format!("-load={}", lib_path.display()));
                    runtime_found = true;
                    break;
                }
            }
            if runtime_found {
                break;
            }
        }
    }

    if !runtime_found {
        eprintln!("Warning: lency_runtime library not found. I/O operations may fail.");
    }

    let output = cmd.arg(temp_ir).output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        if let Some(code) = output.status.code() {
            println!("\n[Program exited with code {}]", code);
        } else {
            eprintln!("\n[Program terminated by signal]");
        }
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}

/// 检查命令
fn cmd_check(input: &str) -> Result<()> {
    println!("Checking {} ...", input);

    let source = fs::read_to_string(input)?;
    match lency_driver::compile(&source) {
        Ok(_) => {
            println!("No errors found");
            Ok(())
        }
        Err(e) => {
            e.emit(Some(input), Some(&source));
            std::process::exit(1);
        }
    }
}

/// 构建命令 - 生成可执行文件
fn cmd_build(input: &str, output: &str, release: bool) -> Result<()> {
    println!("Building {} (release={}) ...", input, release);

    // 1. 编译为 LLVM IR
    let result = compile_file(input)?;
    let temp_ll = "/tmp/lency_temp.ll";
    fs::write(temp_ll, result.ir)?;

    // 2. 使用 llc 生成目标文件
    println!("  Generating object file...");
    let temp_obj = "/tmp/lency_temp.o";
    // TODO: Pass optimization flags to llc if release is true
    let llc_status = std::process::Command::new("llc-15")
        .args(["-filetype=obj", temp_ll, "-o", temp_obj])
        .status()?;

    if !llc_status.success() {
        anyhow::bail!("llc compilation failed");
    }

    // 3. 查找运行时库
    let mut runtime_path = None;
    if let Ok(cwd) = std::env::current_dir() {
        // Defines search order based on release flag
        let dirs = if release {
            vec!["target/release", "target/debug"]
        } else {
            vec!["target/debug", "target/release"]
        };

        // Note: lency_runtime might be compiled as rlib (static) or dylib
        let libs = [
            "liblency_runtime.so",
            "liblency_runtime.dylib",
            "liblency_runtime.a",
        ];

        for dir in dirs {
            for lib in libs {
                let path = cwd.join(dir).join(lib);
                if path.exists() {
                    runtime_path = Some(cwd.join(dir));
                    break;
                }
            }
            if runtime_path.is_some() {
                break;
            }
        }
    }

    if runtime_path.is_none() {
        eprintln!("Warning: lency_runtime library not found in target dir. Linking might fail.");
    }

    // 4. 使用 gcc 链接
    println!("  Linking executable...");

    let mut gcc_cmd = std::process::Command::new("gcc");
    gcc_cmd.args([temp_obj, "-o", output, "-no-pie"]);

    if let Some(path) = runtime_path {
        gcc_cmd.arg(format!("-L{}", path.display()));
        gcc_cmd.arg("-llency_runtime");
        // Add rpath so the binary can find the shared library at runtime
        gcc_cmd.arg(format!("-Wl,-rpath,{}", path.display()));
    }

    let gcc_status = gcc_cmd.status()?;

    if !gcc_status.success() {
        anyhow::bail!("Linking failed - please ensure lency_runtime is built");
    }

    println!("Successfully built: {}", output);
    Ok(())
}

/// REPL 循环 (实验性)
fn cmd_repl() -> Result<()> {
    use std::io::{self, Write};

    println!("Lency REPL (Experimental)");
    println!("Type 'exit' or press Ctrl+D to quit.");

    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break; // EOF
        }

        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        // Just check syntax for now
        // Wrap in a function to allow declarations if needed, or parse as statements
        // But driver compiles a whole file.
        // Let's create a temporary source string.
        match lency_driver::compile(trimmed) {
            Ok(_res) => {
                println!("Parse OK");
                // Optional: Print IR or verify semantic
                // println!("{}", _res.ir);
            }
            Err(e) => {
                // Use enhanced error emission
                // We pass Some(trimmed) as source so snippet is shown
                e.emit(Some("<repl>"), Some(trimmed));
            }
        }
    }

    Ok(())
}
