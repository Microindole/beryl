use anyhow::{bail, Result};
use std::fs;

use crate::path_utils::resolve_output_path;

use super::{
    check::cmd_check,
    common::{compile_to_llvm_ir, find_runtime_library, require_tool, temp_artifact_path},
};

/// 构建命令 - 生成可执行文件
pub fn cmd_build(
    input: &str,
    output: &str,
    out_dir: Option<&str>,
    release: bool,
    check_only: bool,
) -> Result<()> {
    if check_only {
        println!("Building {} (check-only=true) ...", input);
        return cmd_check(input);
    }

    println!("Building {} (release={}) ...", input, release);

    let ir = compile_to_llvm_ir(input)?;
    let temp_ll = temp_artifact_path("ll")?;
    fs::write(&temp_ll, ir)?;

    println!("  Generating object file...");
    let temp_obj = temp_artifact_path("o")?;
    let llc = require_tool(&["llc-15", "llc"], "LLVM static compiler (llc)")?;
    let mut llc_cmd = std::process::Command::new(llc);
    llc_cmd.args(["-filetype=obj"]);
    if release {
        llc_cmd.arg("-O2");
    }
    let llc_status = llc_cmd
        .args([
            temp_ll.to_string_lossy().as_ref(),
            "-o",
            temp_obj.to_string_lossy().as_ref(),
        ])
        .status()?;
    if !llc_status.success() {
        bail!("llc compilation failed");
    }

    let runtime_lib = find_runtime_library();
    if runtime_lib.is_none() {
        eprintln!("Warning: lency_runtime library not found in target dir. Linking might fail.");
    }

    println!("  Linking executable...");
    let output_path = resolve_output_path(output, out_dir)?;
    let output_str = output_path.to_string_lossy().into_owned();

    let linker = require_tool(&["gcc", "clang"], "linker (gcc/clang)")?;
    let mut gcc_cmd = std::process::Command::new(linker);
    gcc_cmd.args([
        temp_obj.to_string_lossy().as_ref(),
        "-o",
        output_str.as_str(),
    ]);
    if !cfg!(windows) {
        gcc_cmd.arg("-no-pie");
    }

    if let Some(lib_path) = runtime_lib {
        if cfg!(windows) {
            // On Windows (MSVC/clang), link directly with the import/static library file.
            gcc_cmd.arg(&lib_path);
        } else if let Some(path) = lib_path.parent() {
            gcc_cmd.arg(format!("-L{}", path.display()));
            gcc_cmd.arg("-llency_runtime");
            gcc_cmd.arg(format!("-Wl,-rpath,{}", path.display()));
        }
    }

    let gcc_status = gcc_cmd.status()?;
    if !gcc_status.success() {
        bail!("Linking failed - please ensure lency_runtime is built");
    }

    let _ = fs::remove_file(&temp_ll);
    let _ = fs::remove_file(&temp_obj);

    println!("Successfully built: {}", output_path.display());
    Ok(())
}
