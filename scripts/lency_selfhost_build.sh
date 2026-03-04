#!/bin/bash
set -euo pipefail

# Build a .lcy source through self-host lencyc pipeline:
#   .lcy --(self-host main --emit-lir)--> .lir --(Rust lencyc build)--> executable

RUST_LENCY_BUILD_CMD="cargo build --release -p lency_cli -p lency_runtime"
RUST_LENCY_EXEC="target/release/lencyc"
SELF_HOST_MAIN_ENTRY="lencyc/driver/main.lcy"
SELF_HOST_OUT_DIR="target/lencyc_selfhost"
SELF_HOST_MAIN_BIN_NAME="lencyc_main"
SELF_HOST_MAIN_BIN="$SELF_HOST_OUT_DIR/$SELF_HOST_MAIN_BIN_NAME"

usage() {
    cat <<'EOF'
Usage:
  ./scripts/lency_selfhost_build.sh <input.lcy> [-o output] [--out-dir DIR] [--check-only] [--release]

Examples:
  ./scripts/lency_selfhost_build.sh tests/example/lencyc_lir_exit0.lcy
  ./scripts/lency_selfhost_build.sh tests/example/lencyc_lir_exit0.lcy -o app --out-dir target/bin --release
  ./scripts/lency_selfhost_build.sh tests/example/lencyc_lir_exit0.lcy --check-only
EOF
}

if [[ $# -lt 1 ]]; then
    usage
    exit 1
fi

INPUT_FILE=""
OUTPUT_NAME=""
OUT_DIR=""
CHECK_ONLY=false
RELEASE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        -o|--output)
            if [[ $# -lt 2 ]]; then
                echo "error: $1 requires a value" >&2
                exit 1
            fi
            OUTPUT_NAME="$2"
            shift 2
            ;;
        --out-dir)
            if [[ $# -lt 2 ]]; then
                echo "error: --out-dir requires a value" >&2
                exit 1
            fi
            OUT_DIR="$2"
            shift 2
            ;;
        --check-only)
            CHECK_ONLY=true
            shift
            ;;
        --release)
            RELEASE=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo "error: unknown option: $1" >&2
            usage
            exit 1
            ;;
        *)
            if [[ -n "$INPUT_FILE" ]]; then
                echo "error: multiple input files are not supported: '$INPUT_FILE' and '$1'" >&2
                exit 1
            fi
            INPUT_FILE="$1"
            shift
            ;;
    esac
done

if [[ -z "$INPUT_FILE" ]]; then
    echo "error: missing input file" >&2
    usage
    exit 1
fi

if [[ ! -f "$INPUT_FILE" ]]; then
    echo "error: input file not found: $INPUT_FILE" >&2
    exit 1
fi

mkdir -p "$SELF_HOST_OUT_DIR"

if [[ -z "$OUTPUT_NAME" ]]; then
    base_name="$(basename "$INPUT_FILE" .lcy)"
    OUTPUT_NAME="${base_name}.out"
fi

if [[ -z "$OUT_DIR" ]]; then
    OUT_DIR="$SELF_HOST_OUT_DIR"
fi
mkdir -p "$OUT_DIR"

echo "[1/4] building rust host compiler ..."
$RUST_LENCY_BUILD_CMD > /dev/null

echo "[2/4] building self-host compiler entry ..."
$RUST_LENCY_EXEC build "$SELF_HOST_MAIN_ENTRY" -o "$SELF_HOST_MAIN_BIN_NAME" --out-dir "$SELF_HOST_OUT_DIR" > /dev/null

emit_name="$(basename "$INPUT_FILE" .lcy).selfhost.lir"
emit_path="$SELF_HOST_OUT_DIR/$emit_name"

echo "[3/4] emitting LIR from self-host compiler ..."
"./$SELF_HOST_MAIN_BIN" "$INPUT_FILE" --emit-lir -o "$emit_path" > /dev/null

echo "[4/4] building executable from emitted LIR ..."
build_cmd=("$RUST_LENCY_EXEC" build "$emit_path" -o "$OUTPUT_NAME" --out-dir "$OUT_DIR")
if [[ "$CHECK_ONLY" == "true" ]]; then
    build_cmd+=("--check-only")
fi
if [[ "$RELEASE" == "true" ]]; then
    build_cmd+=("--release")
fi
"${build_cmd[@]}" > /dev/null

if [[ "$CHECK_ONLY" == "true" ]]; then
    echo "self-host check-only passed: $INPUT_FILE"
else
    echo "self-host build succeeded: $OUT_DIR/$OUTPUT_NAME"
fi

# TODO: support forwarding custom runtime/linker flags to the Rust backend build stage.
