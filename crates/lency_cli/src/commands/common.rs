use crate::lir_backend;
use anyhow::Result;
use lency_driver::compile_file;
use std::{fs, path::PathBuf};

pub fn compile_to_llvm_ir(input: &str) -> Result<String> {
    if input.ends_with(".lir") {
        let source = fs::read_to_string(input)?;
        return lir_backend::compile_lir_to_llvm_ir(&source);
    }
    Ok(compile_file(input)?.ir)
}

pub fn find_runtime_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let dirs = ["target/release", "target/debug"];
    let libs = [
        "liblency_runtime.so",
        "liblency_runtime.dylib",
        "liblency_runtime.a",
    ];

    for dir in dirs {
        for lib in libs {
            let path = cwd.join(dir).join(lib);
            if path.exists() {
                return Some(cwd.join(dir));
            }
        }
    }
    None
}
