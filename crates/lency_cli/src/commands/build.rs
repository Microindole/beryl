use anyhow::{bail, Result};
use std::fs;

use crate::path_utils::resolve_output_path;

use super::{check::cmd_check, common::compile_to_llvm_ir, common::find_runtime_dir};

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
    let temp_ll = "/tmp/lency_temp.ll";
    fs::write(temp_ll, ir)?;

    println!("  Generating object file...");
    let temp_obj = "/tmp/lency_temp.o";
    let mut llc_cmd = std::process::Command::new("llc-15");
    llc_cmd.args(["-filetype=obj"]);
    if release {
        llc_cmd.arg("-O2");
    }
    let llc_status = llc_cmd.args([temp_ll, "-o", temp_obj]).status()?;
    if !llc_status.success() {
        bail!("llc compilation failed");
    }

    if find_runtime_dir().is_none() {
        eprintln!("Warning: lency_runtime library not found in target dir. Linking might fail.");
    }

    println!("  Linking executable...");
    let output_path = resolve_output_path(output, out_dir)?;
    let output_str = output_path.to_string_lossy().into_owned();

    let mut gcc_cmd = std::process::Command::new("gcc");
    gcc_cmd.args([temp_obj, "-o", output_str.as_str(), "-no-pie"]);

    if let Some(path) = find_runtime_dir() {
        gcc_cmd.arg(format!("-L{}", path.display()));
        gcc_cmd.arg("-llency_runtime");
        gcc_cmd.arg(format!("-Wl,-rpath,{}", path.display()));
    }

    let gcc_status = gcc_cmd.status()?;
    if !gcc_status.success() {
        bail!("Linking failed - please ensure lency_runtime is built");
    }

    println!("Successfully built: {}", output_path.display());
    Ok(())
}
