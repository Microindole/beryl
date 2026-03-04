use crate::path_utils::resolve_output_path;
use anyhow::Result;
use std::fs;

use super::common::compile_to_llvm_ir;

/// 编译命令
pub fn cmd_compile(input: &str, output: &str, out_dir: Option<&str>) -> Result<()> {
    println!("Compiling {} ...", input);

    let result_ir = compile_to_llvm_ir(input)?;
    let output_path = resolve_output_path(output, out_dir)?;
    fs::write(&output_path, result_ir)?;
    println!("Generated {}", output_path.display());

    Ok(())
}
