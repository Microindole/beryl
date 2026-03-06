#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 0 ]]; then
  echo "run_checks.sh 不接受参数。该脚本固定为 Rust 专用检查。"
  exit 1
fi

cargo run -p xtask -- check-rust
