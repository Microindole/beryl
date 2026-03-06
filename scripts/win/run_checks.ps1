Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ($args.Count -ne 0) {
    throw "run_checks.ps1 不接受参数。该脚本固定为 Rust 专用检查。"
}

cargo run -p xtask -- check-rust
