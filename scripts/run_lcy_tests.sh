#!/bin/bash
# 运行所有 .lcy 集成测试
# 此脚本用于验证语言特性没有在修复 bug 时被破坏

set -e

echo "🧪 Running .lcy integration tests..."
echo "====================================="

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

PASS=0
FAIL=0
FAILED_FILES=()

# 查找所有 .lcy 文件
LCY_FILES=$(find "$PROJECT_ROOT/tests/integration" -name "*.lcy" | sort)

if [ -z "$LCY_FILES" ]; then
    echo "⚠️  No .lcy files found in tests/integration"
    exit 0
fi

for file in $LCY_FILES; do
    rel_path="${file#$PROJECT_ROOT/}"
    
    # 使用 lencyc check 进行语义检查
    if cargo run --bin lencyc --quiet -- check "$file" > /dev/null 2>&1; then
        echo "✅ $rel_path"
        ((PASS++)) || true
    else
        echo "❌ $rel_path"
        FAILED_FILES+=("$rel_path")
        ((FAIL++)) || true
    fi
done

echo ""
echo "====================================="
echo "📊 Results: $PASS passed, $FAIL failed"

if [ $FAIL -gt 0 ]; then
    echo ""
    echo "❌ Failed files:"
    for f in "${FAILED_FILES[@]}"; do
        echo "   - $f"
    done
    exit 1
fi

echo "✅ All .lcy tests passed!"
