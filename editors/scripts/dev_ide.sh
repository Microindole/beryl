#!/bin/bash

# Lency IDE 开发模式启动脚本 (V6 Professional 版)
# 集成了自动构建流程。

EDITORS_ROOT=$(cd "$(dirname "$0")/.." && pwd)
REPO_ROOT=$(cd "$EDITORS_ROOT/.." && pwd)
EXT_PATH="$EDITORS_ROOT/vscode"

# 1. 尝试自动编译 TypeScript (如果环境支持)
if command -v npm >/dev/null 2>&1; then
    echo "📦 正在编译扩展源码..."
    cd "$EXT_PATH" && npm install --silent && npm run build --silent
    cd "$REPO_ROOT"
fi

# 2. 检查编译产物
if [ ! -f "$EXT_PATH/dist/extension.js" ]; then
    echo "⚠️ 警告: 未找到编译产物 ($EXT_PATH/dist/extension.js)。"
    echo "如果是通过 VSCode 运行，请确保您在主窗口运行了 'npm run build'。"
fi

# 3. 确定编辑器命令
if command -v code >/dev/null 2>&1; then
    IDE_CMD="code"
elif command -v cursor >/dev/null 2>&1; then
    IDE_CMD="cursor"
elif command -v antigravity >/dev/null 2>&1; then
    IDE_CMD="antigravity"
else
    echo "❌ 错误: 未找到 IDE 命令。"
    exit 1
fi

echo "🚀 正在以 Professional 模式启动 $IDE_CMD..."
$IDE_CMD --extensionDevelopmentPath "$EXT_PATH" "$REPO_ROOT"
