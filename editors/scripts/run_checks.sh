#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
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

if [[ "$#" -ne 0 ]]; then
    echo -e "${RED}run_checks.sh 不接受参数。该脚本固定为 Editors 专用检查。${NC}"
    exit 1
fi

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}   Starting Editors-side Checks   ${NC}"
echo -e "${BLUE}======================================${NC}"

print_step "1. Building VSCode Extension"
if "$SCRIPT_DIR/check_editor.sh"; then
    print_success "Editor extension build"
else
    print_error "Editor extension build"
    exit 1
fi

print_step "2. Scanning TODO/FIXME in editors/"
python3 "$SCRIPT_DIR/check_todos.py"
print_success "TODO/FIXME scan"

print_step "3. Checking file sizes in editors/"
if python3 "$SCRIPT_DIR/check_file_size.py"; then
    print_success "File size check"
else
    print_error "File size check"
    exit 1
fi

print_step "4. Checking banned patterns in editors/"
if python3 "$SCRIPT_DIR/check_banned_patterns.py"; then
    print_success "Code quality check"
else
    print_error "Code quality check"
    echo -e "${YELLOW}Please remove banned patterns or add documented exemptions.${NC}"
    exit 1
fi

echo -e "\n${BLUE}======================================${NC}"
echo -e "${GREEN}🎉 All editors checks passed!${NC}"
echo -e "${BLUE}======================================${NC}"
