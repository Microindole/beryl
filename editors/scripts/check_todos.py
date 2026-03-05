#!/usr/bin/env python3
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXTS = {'.ts', '.md', '.json', '.sh', '.py'}
EXCLUDES = {'node_modules', 'dist', '.git', '.vscode'}
TAGS = ('TODO', 'FIXME')

entries = {tag: [] for tag in TAGS}
for path in ROOT.rglob('*'):
    if not path.is_file() or path.suffix not in EXTS:
        continue
    if any(part in EXCLUDES for part in path.parts):
        continue
    text = path.read_text(encoding='utf-8', errors='ignore')
    for idx, line in enumerate(text.splitlines(), 1):
        for tag in TAGS:
            if re.search(rf'\\b{tag}\\b', line):
                entries[tag].append((path.relative_to(ROOT), idx, line.strip()))

print(f"🔍 扫描 Editors TODO/FIXME：{ROOT}")
for tag in TAGS:
    items = entries[tag]
    if not items:
        continue
    icon = '🔴' if tag == 'FIXME' else '📝'
    print(f"{icon} Found {len(items)} {tag}s:")
    for rel, line_no, content in items:
        snippet = content if len(content) <= 80 else content[:77] + '...'
        print(f"   {rel}:{line_no:<4} {snippet}")

total = sum(len(v) for v in entries.values())
print(f"📊 总计发现 {total} 个标记。")
sys.exit(0)
