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
    )
    # FIXME: 恢复对 lencyc/driver/main.lcy 的 --check-only 检查，当前主入口尚未自举完备。
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
    # FIXME: lency_cli::build 子命令尚未实现 --check-only，当前仅能通过完整 build 间接覆盖语法。
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

echo -e "\n${BLUE}=====================================${NC}"
echo -e "${GREEN}🎉 All self-hosted checks passed!${NC}"
echo -e "${BLUE}=====================================${NC}"
