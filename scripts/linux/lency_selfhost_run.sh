#!/usr/bin/env bash
set -euo pipefail

cargo run -p xtask -- selfhost-run "$@"
