#!/usr/bin/env python3
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WARN = 300
ERROR = 500
EXTS = {'.ts', '.py', '.sh'}
EXCLUDES = {'node_modules', 'dist', '.git', '.vscode'}

warnings = []
errors = []
files = []

for path in ROOT.rglob('*'):
    if not path.is_file() or path.suffix not in EXTS:
        continue
    if any(part in EXCLUDES for part in path.parts):
        continue
    lines = [line for line in path.read_text(encoding='utf-8', errors='ignore').splitlines() if line.strip()]
    count = len(lines)
    files.append((path.relative_to(ROOT), count))
    if count > ERROR:
        errors.append((path.relative_to(ROOT), count))
    elif count > WARN:
        warnings.append((path.relative_to(ROOT), count))

print(f"🔍 扫描 Editors 代码文件：{ROOT}")
print(f"   警告阈值: {WARN} 行")
print(f"   错误阈值: {ERROR} 行")
if errors:
    print("❌ 错误：以下文件过大:")
    for rel, count in sorted(errors, key=lambda x: x[1], reverse=True):
        print(f"   {rel}: {count} 行")
if warnings:
    print("⚠️  警告：以下文件偏大:")
    for rel, count in sorted(warnings, key=lambda x: x[1], reverse=True):
        print(f"   {rel}: {count} 行")
print(f"📊 统计: 总文件数={len(files)}, 警告={len(warnings)}, 错误={len(errors)}")
sys.exit(1 if errors else 0)
