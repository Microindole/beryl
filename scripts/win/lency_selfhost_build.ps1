Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

cargo run -p xtask -- selfhost-build @args
