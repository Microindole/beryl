use crate::helpers::{resolve_exec, run_cmd, run_cmd_exit_code};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct SelfhostBuildOptions {
    pub(crate) input_file: PathBuf,
    pub(crate) output_name: String,
    pub(crate) out_dir: PathBuf,
    pub(crate) check_only: bool,
    pub(crate) release: bool,
}

#[derive(Debug)]
pub(crate) struct SelfhostRunOptions {
    pub(crate) input_file: PathBuf,
    pub(crate) out_dir: PathBuf,
    pub(crate) release: bool,
    pub(crate) expect_exit: Option<i32>,
    pub(crate) program_args: Vec<String>,
}

pub(crate) fn selfhost_build_from_args(args: &[String]) -> Result<()> {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_selfhost_build_usage();
        return Ok(());
    }

    let mut input_file: Option<PathBuf> = None;
    let mut output_name: Option<String> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut check_only = false;
    let mut release = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("{} requires a value", args[i]);
                };
                output_name = Some(v.clone());
                i += 2;
            }
            "--out-dir" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("--out-dir requires a value");
                };
                out_dir = Some(PathBuf::from(v));
                i += 2;
            }
            "--check-only" => {
                check_only = true;
                i += 1;
            }
            "--release" => {
                release = true;
                i += 1;
            }
            s if s.starts_with('-') => bail!("unknown option: {s}"),
            other => {
                if input_file.is_some() {
                    bail!("multiple input files are not supported");
                }
                input_file = Some(PathBuf::from(other));
                i += 1;
            }
        }
    }

    let input_file = input_file.context("missing input file")?;
    let output_name = output_name.unwrap_or_else(|| {
        input_file
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| format!("{s}.out"))
            .unwrap_or_else(|| "a.out".to_string())
    });
    let out_dir = out_dir.unwrap_or_else(|| PathBuf::from("target/lencyc_selfhost"));

    let opts = SelfhostBuildOptions {
        input_file,
        output_name,
        out_dir,
        check_only,
        release,
    };

    let result = selfhost_build_impl(&opts)?;
    if opts.check_only {
        println!("self-host check-only passed: {}", opts.input_file.display());
    } else {
        println!("self-host build succeeded: {}", result.display());
    }
    Ok(())
}

pub(crate) fn selfhost_run_from_args(args: &[String]) -> Result<()> {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_selfhost_run_usage();
        return Ok(());
    }

    let mut input_file: Option<PathBuf> = None;
    let mut out_dir = PathBuf::from("target/lencyc_selfhost");
    let mut release = false;
    let mut expect_exit: Option<i32> = None;
    let mut program_args: Vec<String> = Vec::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--release" => {
                release = true;
                i += 1;
            }
            "--out-dir" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("--out-dir requires a value");
                };
                out_dir = PathBuf::from(v);
                i += 2;
            }
            "--expect-exit" => {
                let Some(v) = args.get(i + 1) else {
                    bail!("--expect-exit requires a value");
                };
                expect_exit = Some(
                    v.parse::<i32>()
                        .with_context(|| format!("invalid --expect-exit value: {v}"))?,
                );
                i += 2;
            }
            "--" => {
                program_args.extend(args[(i + 1)..].iter().cloned());
                break;
            }
            s if s.starts_with('-') => bail!("unknown option: {s}"),
            other => {
                if input_file.is_none() {
                    input_file = Some(PathBuf::from(other));
                } else {
                    program_args.push(other.to_string());
                }
                i += 1;
            }
        }
    }

    let input_file = input_file.context("missing input file")?;
    let opts = SelfhostRunOptions {
        input_file,
        out_dir,
        release,
        expect_exit,
        program_args,
    };

    selfhost_run_impl(&opts)
}

pub(crate) fn selfhost_build_impl(opts: &SelfhostBuildOptions) -> Result<PathBuf> {
    let rust_lency_exec = prepare_rust_lency_cli()?;
    let self_host_out_dir = Path::new("target/lencyc_selfhost");
    let self_host_main_entry = Path::new("lencyc/driver/main.lcy");
    let self_host_main_out_name = "lencyc_main";

    if !opts.input_file.exists() {
        bail!("input file not found: {}", opts.input_file.display());
    }

    fs::create_dir_all(self_host_out_dir)
        .with_context(|| format!("failed to create {}", self_host_out_dir.display()))?;
    fs::create_dir_all(&opts.out_dir)
        .with_context(|| format!("failed to create {}", opts.out_dir.display()))?;

    println!("[1/4] building rust host compiler ...");
    println!("[2/4] building self-host compiler entry ...");
    run_cmd(
        &rust_lency_exec,
        &[
            "build",
            &self_host_main_entry.to_string_lossy(),
            "-o",
            self_host_main_out_name,
            "--out-dir",
            &self_host_out_dir.to_string_lossy(),
        ],
        true,
        &[],
        &[0],
    )?;

    let self_host_main = resolve_exec(&self_host_out_dir.join(self_host_main_out_name))?;
    let emit_name = format!(
        "{}.selfhost.lir",
        opts.input_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
    );
    let emit_path = self_host_out_dir.join(emit_name);

    println!("[3/4] emitting LIR from self-host compiler ...");
    run_cmd(
        &self_host_main,
        &[
            &opts.input_file.to_string_lossy(),
            "--emit-lir",
            "-o",
            &emit_path.to_string_lossy(),
        ],
        true,
        &[],
        &[0],
    )?;

    println!("[4/4] building executable from emitted LIR ...");
    let mut build_args = vec![
        "build".to_string(),
        emit_path.to_string_lossy().to_string(),
        "-o".to_string(),
        opts.output_name.clone(),
        "--out-dir".to_string(),
        opts.out_dir.to_string_lossy().to_string(),
    ];
    if opts.check_only {
        build_args.push("--check-only".to_string());
    }
    if opts.release {
        build_args.push("--release".to_string());
    }

    let build_args_ref: Vec<&str> = build_args.iter().map(String::as_str).collect();
    run_cmd(&rust_lency_exec, &build_args_ref, true, &[], &[0])?;

    Ok(opts.out_dir.join(&opts.output_name))
}

pub(crate) fn selfhost_run_impl(opts: &SelfhostRunOptions) -> Result<()> {
    let output_name = format!(
        "{}.run.out",
        opts.input_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
    );

    let build_opts = SelfhostBuildOptions {
        input_file: opts.input_file.clone(),
        output_name: output_name.clone(),
        out_dir: opts.out_dir.clone(),
        check_only: false,
        release: opts.release,
    };

    println!("[1/2] building self-host executable ...");
    let output_path = selfhost_build_impl(&build_opts)?;

    println!("[2/2] running executable ...");
    let exe = resolve_exec(&output_path)?;
    let run_args: Vec<&str> = opts.program_args.iter().map(String::as_str).collect();
    let code = run_cmd_exit_code(&exe, &run_args, false)?;

    if let Some(expected) = opts.expect_exit {
        if code != expected {
            bail!("exit code mismatch, expected {}, got {}", expected, code);
        }
        println!("self-host run succeeded: expected exit code {}", expected);
        return Ok(());
    }

    println!("self-host run exit code: {}", code);
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}

pub(crate) fn prepare_rust_lency_cli() -> Result<PathBuf> {
    run_cmd(
        "cargo",
        &[
            "build",
            "--release",
            "-p",
            "lency_cli",
            "-p",
            "lency_runtime",
        ],
        true,
        &[],
        &[0],
    )?;
    let base = Path::new("target").join("release").join("lencyc");
    resolve_exec(&base)
}

fn print_selfhost_build_usage() {
    eprintln!("Usage:");
    eprintln!(
        "  cargo run -p xtask -- selfhost-build <input.lcy> [-o output] [--out-dir DIR] [--check-only] [--release]"
    );
}

fn print_selfhost_run_usage() {
    eprintln!("Usage:");
    eprintln!(
        "  cargo run -p xtask -- selfhost-run <input.lcy> [--release] [--out-dir DIR] [--expect-exit N] [--] [program args...]"
    );
}
