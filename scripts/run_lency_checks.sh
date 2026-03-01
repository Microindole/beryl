#!/bin/bash
set -e

# Configuration
# 构建 Rust Lency CLI 的命令
RUST_LENCY_BUILD_CMD="cargo build --release -p lency_cli -p lency_runtime"
RUST_LENCY_EXEC="target/release/lencyc"

# 测试 Lency 自举编译器的入口文件
SELF_HOST_ENTRY="lencyc/driver/main.lcy"
# 输出的可执行文件名称
SELF_HOST_OUT="lencyc_compiler"

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

echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}   Starting Lency Self-host Checks   ${NC}"
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
python3 scripts/check_todos.py
# 检查文件大小
python3 scripts/check_file_size.py
# 检查 Lencyc 专用规范 (命名等)
if python3 scripts/check_lencyc_meta.py; then
    print_success "Meta checks"
else
    print_error "Meta checks"
    exit 1
fi

# 2. 使用 Rust 编译器编译 Lency 的自举版 (目前只有前端解析)
print_step "2. Compiling Lency-written Compiler (Self-host Lencyc)"
if [ ! -f "$SELF_HOST_ENTRY" ]; then
    print_error "Cannot find self-host entry file: $SELF_HOST_ENTRY"
    exit 1
fi

if $RUST_LENCY_EXEC build $SELF_HOST_ENTRY -o $SELF_HOST_OUT; then
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
