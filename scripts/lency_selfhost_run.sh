#!/bin/bash
set -euo pipefail

# Run a .lcy source through self-host pipeline:
#   .lcy --(self-host emit-lir + Rust build)--> executable --(run with args)

usage() {
    cat <<'EOF'
Usage:
  ./scripts/lency_selfhost_run.sh <input.lcy> [--release] [--out-dir DIR] [--expect-exit N] [--] [program args...]

Examples:
  ./scripts/lency_selfhost_run.sh tests/example/lencyc_lir_exit0.lcy
  ./scripts/lency_selfhost_run.sh tests/example/lencyc_run_args.lcy --expect-exit 1 -- foo
  ./scripts/lency_selfhost_run.sh tests/example/lencyc_lir_exit0.lcy --release --out-dir target/bin
EOF
}

if [[ $# -lt 1 ]]; then
    usage
    exit 1
fi

INPUT_FILE=""
OUT_DIR="target/lencyc_selfhost"
RELEASE=false
EXPECT_EXIT=""
PROGRAM_ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --release)
            RELEASE=true
            shift
            ;;
        --out-dir)
            if [[ $# -lt 2 ]]; then
                echo "error: --out-dir requires a value" >&2
                exit 1
            fi
            OUT_DIR="$2"
            shift 2
            ;;
        --expect-exit)
            if [[ $# -lt 2 ]]; then
                echo "error: --expect-exit requires a value" >&2
                exit 1
            fi
            EXPECT_EXIT="$2"
            shift 2
            ;;
        --)
            shift
            PROGRAM_ARGS=("$@")
            break
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
                PROGRAM_ARGS+=("$1")
            else
                INPUT_FILE="$1"
            fi
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

mkdir -p "$OUT_DIR"
base_name="$(basename "$INPUT_FILE" .lcy)"
output_name="${base_name}.run.out"
output_path="$OUT_DIR/$output_name"

build_cmd=("./scripts/lency_selfhost_build.sh" "$INPUT_FILE" "-o" "$output_name" "--out-dir" "$OUT_DIR")
if [[ "$RELEASE" == "true" ]]; then
    build_cmd+=("--release")
fi

echo "[1/2] building self-host executable ..."
"${build_cmd[@]}" > /dev/null

echo "[2/2] running executable ..."
set +e
"$output_path" "${PROGRAM_ARGS[@]}"
run_status=$?
set -e

if [[ -n "$EXPECT_EXIT" ]]; then
    if [[ "$run_status" -ne "$EXPECT_EXIT" ]]; then
        echo "error: exit code mismatch, expected $EXPECT_EXIT, got $run_status" >&2
        exit 1
    fi
    echo "self-host run succeeded: expected exit code $EXPECT_EXIT"
    exit 0
fi

echo "self-host run exit code: $run_status"
exit "$run_status"
