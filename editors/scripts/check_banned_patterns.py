#!/usr/bin/env python3
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXCLUDES = {'node_modules', 'dist', '.git', '.vscode', 'test'}

patterns = [
    (re.compile(r'\bconsole\.log\('), 'warning', 'Avoid console.log in extension source.'),
    (re.compile(r'\bdebugger\b'), 'error', 'Found debugger statement.'),
]

issues = []
for path in ROOT.rglob('*.ts'):
    if any(part in EXCLUDES for part in path.parts):
        continue
    lines = path.read_text(encoding='utf-8', errors='ignore').splitlines()
    for i, line in enumerate(lines, 1):
        if line.strip().startswith('//'):
            continue
        for pattern, level, msg in patterns:
            if pattern.search(line):
                issues.append((path.relative_to(ROOT), i, level, msg, line.strip()))

print(f"🔍 Running Editors code quality checks: {ROOT}")
errors = 0
warnings = 0
for rel, line_no, level, msg, code in issues:
    icon = '❌' if level == 'error' else '⚠️ '
    print(f"{icon} {rel}:{line_no} - {msg}")
    print(f"    Code: {code}")
    if level == 'error':
        errors += 1
    else:
        warnings += 1

if errors:
    print(f"❌ Failed: Found {errors} errors.")
    sys.exit(1)
print("✅ No banned code patterns found.")
if warnings:
    print(f"⚠️  Warnings: {warnings}")
sys.exit(0)
