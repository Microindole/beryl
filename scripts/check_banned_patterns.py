#!/usr/bin/env python3
"""
代码质量检查脚本
用于检测不推荐的代码模式，如 unwrap(), expect(), panic!() 等。
"""

import os
import sys
import re
from pathlib import Path
from typing import List, Tuple, Dict

# 需要检查的根目录
CHECK_DIRS = ['crates', 'lib'] # Include 'lib' for .lcy files
# 排除的文件或目录
EXCLUDE_DIRS = {'tests', 'target', 'node_modules', '.git', 'examples'}
EXCLUDE_FILES = {'lency_cli/src/main.rs'} # CLI 入口允许 println

# 定义禁止的模式
# (Pattern, Message, Severity)
BANNED_PATTERNS = [
    # Rust Patterns
    (re.compile(r'\.unwrap\(\)'), "Avoid `.unwrap()` in production code. Use `?` or `match`.", 'error'),
    (re.compile(r'\.expect\('), "Avoid `.expect()` in production code. Use proper error handling.", 'error'),
    (re.compile(r'\bpanic!\('), "Avoid `panic!()`. Return `Result` instead.", 'error'),
    (re.compile(r'\bprintln!\('), "Avoid `println!()` in library code. Use proper logging or diagnostics.", 'warning'),
    (re.compile(r'\btodo!\('), "Found unfinished code `todo!()`.", 'error'),
    (re.compile(r'\bdbbg!\('), "Found debug macro `dbg!()`.", 'error'),
    
    # Lency Patterns (Checking .lcy files in lib/)
    # Lency doesn't have macros like panic!, but we might want to check specialized things or TODOs
    # Currently check_todos.py handles TODOs.
    # We can check for 'null' assignment if we want to be strict, but that's valid code.
    # Maybe check for 'print' in core libraries if we want to enforce structure?
    # For now, let's keep it simple and just ensure we scan.
]

# 在这些路径下放宽检查
# (File Pattern, Allowed Rules)
EXEMPTIONS = [
    (r'lency_runtime', {'error'}), # Runtime allows panics/unwraps (OOM, FFI)
    (r'lency_codegen', {'error'}), # Legacy: LLVM calls use unwrap heavily
    (r'lency_syntax', {'error'}),  # Legacy: Parser internals
    (r'lency_driver', {'error'}),  # Legacy: Driver logic
    (r'lency_diagnostics', {'warning'}), # Diagnostics uses println
    (r'tests.rs', {'error', 'warning'}),
    (r'test.rs', {'error', 'warning'}),
    (r'/tests/', {'error', 'warning'}),
    (r'test_', {'error', 'warning'}),
    (r'\.lcy$', {'error', 'warning'}), # Currently Lency code doesn't have these rust-specific banned patterns, but file scanning logic needs update
]


def check_file(file_path: Path) -> List[Tuple[int, str, str, str]]:
    """检查单个文件，返回 (line_num, line_content, message, severity)"""
    issues = []
    path_str = str(file_path)

    # 检查豁免规则
    allowed_severities = set()
    for pattern, allowed in EXEMPTIONS:
        if pattern in path_str or re.search(pattern, path_str):
            allowed_severities.update(allowed)
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
            
        for i, line in enumerate(lines, 1):
            line_stripped = line.strip()
            if line_stripped.startswith('//'): # 忽略注释
                continue
                
            # 允许通过注释豁免: // allow: unwrap
            if '// allow:' in line:
                continue
            
            # 允许在 assert 中使用 unwrap (测试代码常见)
            if 'assert' in line and ('unwrap' in line or 'expect' in line):
                continue

            for pattern, msg, severity in BANNED_PATTERNS:
                if severity in allowed_severities:
                    continue

                if pattern.search(line):
                    # 特殊情况：lency_cli 允许 println
                    if 'lency_cli' in path_str and 'println!' in line:
                        continue
                        
                    issues.append((i, line_stripped, msg, severity))
                    
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        
    return issues

def main():
    root_dir = Path.cwd()
    all_issues = []
    
    print(f"🔍 Running Code Quality Checks in: {CHECK_DIRS}")
    
    for check_dir in CHECK_DIRS:
        start_path = root_dir / check_dir
        if not start_path.exists():
            continue
            
        for root, dirs, files in os.walk(start_path):
            # 过滤排除目录
            dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
            
            for file in files:
                if file.endswith('.rs') or file.endswith('.lcy'):
                    file_path = Path(root) / file
                    # 检查是否排除
                    if any(str(file_path).endswith(ex) for ex in EXCLUDE_FILES):
                        continue
                        
                    file_issues = check_file(file_path)
                    for ln, content, msg, level in file_issues:
                        all_issues.append((file_path.relative_to(root_dir), ln, content, msg, level))

    error_count = 0
    warning_count = 0
    
    if all_issues:
        print("\nFound issues:")
        for path, ln, content, msg, level in all_issues:
            icon = "❌" if level == 'error' else "⚠️ "
            print(f"{icon} {path}:{ln} - {msg}")
            print(f"    Code: {content}")
            
            if level == 'error':
                error_count += 1
            else:
                warning_count += 1
        print()
        
    if error_count > 0:
        print(f"❌ Failed: Found {error_count} code pattern violations.")
        sys.exit(1)
    elif warning_count > 0:
        print(f"⚠️  Passed with {warning_count} warnings.")
        sys.exit(0)
    else:
        print("✅ No banned code patterns found.")
        sys.exit(0)

if __name__ == '__main__':
    main()
