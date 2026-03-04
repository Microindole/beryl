use anyhow::{anyhow, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn resolve_output_path(output: &str, out_dir: Option<&str>) -> Result<PathBuf> {
    let output_path = PathBuf::from(output);

    if let Some(dir) = out_dir {
        let dir_path = Path::new(dir);
        fs::create_dir_all(dir_path)?;

        let file_name = output_path
            .file_name()
            .ok_or_else(|| anyhow!("Invalid output file name: {}", output))?;

        return Ok(dir_path.join(file_name));
    }

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(output_path)
}
