#!/bin/bash
set -e

# Configuration
# 构建 Rust Lency CLI 的命令
RUST_LENCY_BUILD_CMD="cargo build --release -p lency_cli -p lency_runtime"
RUST_LENCY_EXEC="target/release/lencyc"

# 测试 Lency 自举编译器的入口文件 (用于完整性测试)
SELF_HOST_ENTRY="lencyc/driver/test_entry.lcy"
# 输出目录与可执行文件名称（避免产物落在仓库根目录）
SELF_HOST_OUT_DIR="target/lencyc_selfhost"
SELF_HOST_OUT_NAME="lencyc_test"
SELF_HOST_OUT="$SELF_HOST_OUT_DIR/$SELF_HOST_OUT_NAME"
SELF_HOST_MAIN_ENTRY="lencyc/driver/main.lcy"
SELF_HOST_MAIN_OUT_NAME="lencyc_main"
SELF_HOST_MAIN_OUT="$SELF_HOST_OUT_DIR/$SELF_HOST_MAIN_OUT_NAME"
SELF_HOST_MAIN_EMIT="lencyc_selfhost_ast.txt"
LIR_TEST_CASES=(
    "tests/example/lencyc_lir_basic.lcy"
    "tests/example/lencyc_lir_exit0.lcy"
    "tests/example/lencyc_lir_loop_if.lcy"
    "tests/example/lencyc_lir_unary_logic.lcy"
    "tests/example/lencyc_lir_break_continue.lcy"
)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "\n${BLUE}🚀 $1...${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1 passed${NC}"
}

print_error() {
    echo -e "${RED}❌ $1 failed${NC}"
}

META_SCOPE="lency"
if [[ "$#" -ne 0 ]]; then
    echo -e "${RED}run_lency_checks.sh 不接受参数。该脚本固定为 Lency 专用检查。${NC}"
    exit 1
fi

echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}   Starting Lency-side Checks   ${NC}"
echo -e "${BLUE}=====================================${NC}"

# 1. 编译 Rust 宿主编译器
print_step "1. Compiling Rust Host Compiler (lency_cli)"
if $RUST_LENCY_BUILD_CMD; then
    print_success "Rust host compiler build"
else
    print_error "Rust host compiler build"
    exit 1
fi

# 1.5. 代码质量检查 (Meta Checks)
print_step "1.5. Running Meta Checks (TODOs, File Size, Naming)"
# 扫描 TODO/FIXME
python3 scripts/check_todos.py --scope "$META_SCOPE"
# 检查文件大小
python3 scripts/check_file_size.py --scope "$META_SCOPE"
# 检查 Lencyc 专用规范 (命名等)
if python3 scripts/check_lencyc_meta.py; then
    print_success "Meta checks"
else
    print_error "Meta checks"
    exit 1
fi

# 1.6. 入口级语法检查（仅在 CLI 支持 --check-only 时启用）
print_step "1.6. Running Entry Syntax Checks for lencyc/"
if $RUST_LENCY_EXEC build --help | grep -q -- "--check-only"; then
    CHECK_ENTRIES=(
        "lencyc/driver/test_entry.lcy"
        "lencyc/driver/main.lcy"
    )
    for entry in "${CHECK_ENTRIES[@]}"; do
        if [ ! -f "$entry" ]; then
            print_error "Missing check entry: $entry"
            exit 1
        fi
        if ! $RUST_LENCY_EXEC build "$entry" --check-only > /dev/null 2>&1; then
            print_error "Syntax check failed: $entry"
            exit 1
        fi
    done
    print_success "Entry syntax checks"
else
    echo -e "${YELLOW}⚠️ Skipped entry syntax checks: '--check-only' is not supported by current lencyc build command.${NC}"
fi

# 2. 使用 Rust 编译器编译 Lency 的自举版 (验证 test_entry 逻辑)
print_step "2. Compiling Lency-written Compiler (Self-host Lencyc)"
if [ ! -f "$SELF_HOST_ENTRY" ]; then
    print_error "Cannot find self-host entry file: $SELF_HOST_ENTRY"
    exit 1
fi

mkdir -p "$SELF_HOST_OUT_DIR"

if $RUST_LENCY_EXEC build $SELF_HOST_ENTRY -o $SELF_HOST_OUT_NAME --out-dir "$SELF_HOST_OUT_DIR"; then
    print_success "Self-hosted Lencyc compilation"
else
    print_error "Self-hosted Lencyc compilation"
    exit 1
fi

# 3. 运行已编译 of Lencyc 可执行文件并验证
print_step "3. Running Compiled Self-host Lencyc Basic Tests"
if ./$SELF_HOST_OUT; then
    print_success "Self-hosted Lencyc execution test"
else
    print_error "Self-hosted Lencyc execution test"
    exit 1
fi

# 4. 编译并运行自举主入口，验证最小完整流水线
print_step "4. Compiling Self-host Main Pipeline Entry"
if [ ! -f "$SELF_HOST_MAIN_ENTRY" ]; then
    print_error "Cannot find self-host main entry file: $SELF_HOST_MAIN_ENTRY"
    exit 1
fi

if $RUST_LENCY_EXEC build $SELF_HOST_MAIN_ENTRY -o $SELF_HOST_MAIN_OUT_NAME --out-dir "$SELF_HOST_OUT_DIR"; then
    print_success "Self-host main compilation"
else
    print_error "Self-host main compilation"
    exit 1
fi

print_step "5. Running Self-host Main Pipeline"
if ./$SELF_HOST_MAIN_OUT "lencyc/driver/pipeline_sample.lcy" -o "$SELF_HOST_MAIN_EMIT"; then
    print_success "Self-host main execution"
else
    print_error "Self-host main execution"
    exit 1
fi

print_step "6. Verifying Self-host Main Emit Output"
if [ ! -s "$SELF_HOST_MAIN_EMIT" ]; then
    print_error "Self-host main emit output missing or empty: $SELF_HOST_MAIN_EMIT"
    exit 1
fi
if ! grep -q "AST\\[0\\]:" "$SELF_HOST_MAIN_EMIT"; then
    print_error "Self-host main emit output format mismatch: $SELF_HOST_MAIN_EMIT"
    exit 1
fi
print_success "Self-host main emit output"

print_step "7. Running Self-host LIR Emit Regression Cases"
for case_file in "${LIR_TEST_CASES[@]}"; do
    if [ ! -f "$case_file" ]; then
        print_error "Missing LIR test case: $case_file"
        exit 1
    fi

    case_name="$(basename "$case_file" .lcy)"
    case_out="$SELF_HOST_OUT_DIR/${case_name}.lir"
    if ! ./$SELF_HOST_MAIN_OUT "$case_file" --emit-lir -o "$case_out" > /dev/null 2>&1; then
        print_error "Self-host LIR emit failed: $case_file"
        exit 1
    fi

    if [ ! -s "$case_out" ]; then
        print_error "Self-host LIR output missing or empty: $case_out"
        exit 1
    fi
    if ! grep -q "^; lencyc-lir v0" "$case_out"; then
        print_error "Self-host LIR header mismatch: $case_out"
        exit 1
    fi
    if ! grep -q "^func main {" "$case_out"; then
        print_error "Self-host LIR function header missing: $case_out"
        exit 1
    fi
    if ! grep -q "ret" "$case_out"; then
        print_error "Self-host LIR has no return instruction: $case_out"
        exit 1
    fi
done
print_success "Self-host LIR emit regression"

print_step "8. Running Rust LIR->LLVM Build Smoke Test"
LIR_E2E_CASE="tests/example/lencyc_lir_exit0.lcy"
LIR_E2E_OUT="$SELF_HOST_OUT_DIR/lir_e2e_exit0.lir"
LIR_E2E_BIN_NAME="lir_e2e_exit0"
LIR_E2E_BIN="$SELF_HOST_OUT_DIR/$LIR_E2E_BIN_NAME"

if ! ./$SELF_HOST_MAIN_OUT "$LIR_E2E_CASE" --emit-lir -o "$LIR_E2E_OUT" > /dev/null 2>&1; then
    print_error "Failed to emit LIR for Rust backend smoke test: $LIR_E2E_CASE"
    exit 1
fi
if ! $RUST_LENCY_EXEC build "$LIR_E2E_OUT" -o "$LIR_E2E_BIN_NAME" --out-dir "$SELF_HOST_OUT_DIR" > /dev/null 2>&1; then
    print_error "Rust LIR build smoke test failed: $LIR_E2E_OUT"
    exit 1
fi
if ! ./$LIR_E2E_BIN > /dev/null 2>&1; then
    print_error "Rust LIR smoke executable failed: $LIR_E2E_BIN"
    exit 1
fi
print_success "Rust LIR->LLVM smoke test"

print_step "9. Running Self-host One-step Build Flow"
SELFHOST_FLOW_BIN_NAME="selfhost_flow_exit0"
SELFHOST_FLOW_BIN="$SELF_HOST_OUT_DIR/$SELFHOST_FLOW_BIN_NAME"
if ! ./scripts/lency_selfhost_build.sh "$LIR_E2E_CASE" -o "$SELFHOST_FLOW_BIN_NAME" --out-dir "$SELF_HOST_OUT_DIR" > /dev/null 2>&1; then
    print_error "Self-host one-step build flow failed"
    exit 1
fi
if ! ./"$SELFHOST_FLOW_BIN" > /dev/null 2>&1; then
    print_error "Self-host one-step executable failed: $SELFHOST_FLOW_BIN"
    exit 1
fi
print_success "Self-host one-step build flow"

print_step "10. Running Self-host One-step Run Flow"
SELFHOST_RUN_CASE="tests/example/lencyc_run_args.lcy"
if ! ./scripts/lency_selfhost_run.sh "$SELFHOST_RUN_CASE" --expect-exit 1 -- sample_arg > /dev/null 2>&1; then
    print_error "Self-host one-step run flow failed"
    exit 1
fi
print_success "Self-host one-step run flow"

print_step "11. Running Self-host Runtime Builtin Mapping Flow"
SELFHOST_RUNTIME_CASE="tests/example/lencyc_run_int_to_string.lcy"
if ! ./scripts/lency_selfhost_run.sh "$SELFHOST_RUNTIME_CASE" --expect-exit 0 > /dev/null 2>&1; then
    print_error "Self-host runtime builtin mapping flow failed"
    exit 1
fi
print_success "Self-host runtime builtin mapping flow"

echo -e "\n${BLUE}=====================================${NC}"
echo -e "${GREEN}🎉 All self-hosted checks passed!${NC}"
echo -e "${BLUE}=====================================${NC}"
