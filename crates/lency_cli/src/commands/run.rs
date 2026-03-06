use anyhow::Result;
use std::fs;

use super::common::{find_runtime_dir, require_tool, temp_artifact_path};

/// 运行命令
pub fn cmd_run(input: &str) -> Result<()> {
    println!("Running {} ...", input);

    let result = lency_driver::compile_file(input)?;

    let temp_ir = temp_artifact_path("ll")?;
    fs::write(&temp_ir, result.ir)?;

    let lli = require_tool(&["lli-15", "lli"], "LLVM IR interpreter (lli)")?;
    let mut cmd = std::process::Command::new(lli);
    if let Some(runtime_dir) = find_runtime_dir() {
        let libs = ["liblency_runtime.so", "liblency_runtime.dylib"];
        for lib in libs {
            let lib_path = runtime_dir.join(lib);
            if lib_path.exists() {
                cmd.arg(format!("-load={}", lib_path.display()));
                break;
            }
        }
    } else {
        eprintln!("Warning: lency_runtime library not found. I/O operations may fail.");
    }

    let output = cmd.arg(&temp_ir).output()?;

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

    let _ = fs::remove_file(&temp_ir);

    Ok(())
}
