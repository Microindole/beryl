use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub(crate) fn step<F>(name: &str, action: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    println!("\n==> {name}");
    action()?;
    println!("[ok] {name}");
    Ok(())
}

pub(crate) fn run_cmd<P: AsRef<Path>>(
    program: P,
    args: &[&str],
    quiet: bool,
    envs: &[(&str, &str)],
    accept_codes: &[i32],
) -> Result<()> {
    let program = program.as_ref();
    let mut cmd = Command::new(program);
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    if quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = cmd.status().with_context(|| {
        format!(
            "failed to run command: {} {}",
            program.display(),
            args.join(" ")
        )
    })?;
    let code = status.code().unwrap_or(-1);
    if !accept_codes.contains(&code) {
        bail!(
            "command failed with exit code {}: {} {}",
            code,
            program.display(),
            args.join(" ")
        );
    }
    Ok(())
}

pub(crate) fn run_cmd_exit_code<P: AsRef<Path>>(
    program: P,
    args: &[&str],
    quiet: bool,
) -> Result<i32> {
    let program = program.as_ref();
    let mut cmd = Command::new(program);
    cmd.args(args);
    if quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }
    let status = cmd.status().with_context(|| {
        format!(
            "failed to run command: {} {}",
            program.display(),
            args.join(" ")
        )
    })?;
    Ok(status.code().unwrap_or(-1))
}

pub(crate) fn run_cmd_capture(program: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new(program).args(args).output().with_context(|| {
        format!(
            "failed to run command: {} {}",
            program.display(),
            args.join(" ")
        )
    })?;
    if !output.status.success() {
        bail!(
            "command failed with exit code {:?}: {} {}",
            output.status.code(),
            program.display(),
            args.join(" ")
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub(crate) fn resolve_exec(base: &Path) -> Result<PathBuf> {
    let mut candidates = vec![base.to_path_buf()];
    candidates.push(PathBuf::from(format!("{}.exe", base.display())));
    for c in candidates {
        if c.exists() {
            return Ok(c);
        }
    }
    bail!("executable not found: {}(.exe)", base.display())
}

pub(crate) fn ensure_file_non_empty(path: &Path) -> Result<()> {
    let meta = fs::metadata(path).with_context(|| format!("missing file: {}", path.display()))?;
    if meta.len() == 0 {
        bail!("file is empty: {}", path.display());
    }
    Ok(())
}

pub(crate) fn ensure_contains_line_start(path: &Path, prefix: &str) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read: {}", path.display()))?;
    if content.lines().any(|line| line.starts_with(prefix)) {
        return Ok(());
    }
    bail!(
        "expected line starting with '{}' not found in {}",
        prefix,
        path.display()
    )
}

pub(crate) fn ensure_contains_substr(path: &Path, pattern: &str) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read: {}", path.display()))?;
    if content.contains(pattern) {
        return Ok(());
    }
    bail!("expected '{}' not found in {}", pattern, path.display())
}

pub(crate) struct PythonExec {
    pub(crate) program: &'static str,
    pub(crate) prefix_args: &'static [&'static str],
}

pub(crate) fn run_python(python: &PythonExec, script_args: &[&str], quiet: bool) -> Result<()> {
    let mut all_args: Vec<&str> = python.prefix_args.to_vec();
    all_args.extend_from_slice(script_args);
    run_cmd(
        python.program,
        &all_args,
        quiet,
        &[("PYTHONUTF8", "1"), ("PYTHONIOENCODING", "utf-8")],
        &[0],
    )
}

pub(crate) fn detect_python() -> Result<PythonExec> {
    let candidates = [
        PythonExec {
            program: "python3",
            prefix_args: &[],
        },
        PythonExec {
            program: "python",
            prefix_args: &[],
        },
        PythonExec {
            program: "py",
            prefix_args: &["-3"],
        },
    ];

    for candidate in candidates {
        let mut cmd = Command::new(candidate.program);
        cmd.args(candidate.prefix_args);
        let status = cmd
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        if let Ok(s) = status {
            if s.success() {
                return Ok(candidate);
            }
        }
    }
    bail!("python executable not found (tried: python3, python, py -3)")
}
