#!/usr/bin/env python3
"""
Lencyc 专用的元检查脚本
1. 检查命名规范 (snake_case for functions/vars, PascalCase for structs)
2. 检查是否有基本的模块注释
"""
import os
import re
import sys
from pathlib import Path

# 路径配置
LENCYC_DIR = Path("lencyc")
EXCLUDE_DIRS = {".git", "target", "node_modules"}

# 正则表达式
RE_FUNC = re.compile(r'(?:fn|int|string|bool|void|Vec<.*>|Expr|Stmt|Token|Lexer|Parser|Pair<.*>|Box<.*>|Option<.*>)\s+([a-zA-Z0-9_]+)\s*\(')
RE_VAR = re.compile(r'var\s+([a-zA-Z0-9_]+)')
RE_STRUCT = re.compile(r'struct\s+([a-zA-Z0-9_]+)')
RE_ENUM = re.compile(r'enum\s+([a-zA-Z0-9_]+)')

def is_snake_case(name: str) -> bool:
    if not name: return True
    # 允许 T_STAR() 这种全大写常量风格
    if name.isupper(): return True
    return bool(re.match(r'^[a-z][a-z0-9_]*$', name))

def is_pascal_case(name: str) -> bool:
    if not name: return True
    return bool(re.match(r'^[A-Z][a-zA-Z0-9]*$', name))

def check_file(file_path: Path):
    errors = []
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
        lines = content.splitlines()

        # 1. 检查文件头注释
        if lines and not lines[0].startswith("//"):
            errors.append((1, "文件缺少头注释 (//)"))

        for i, line in enumerate(lines, 1):
            # 2. 检查函数命名
            for match in RE_FUNC.finditer(line):
                name = match.group(1)
                if not is_snake_case(name):
                    errors.append((i, f"函数名 '{name}' 应该使用 snake_case"))

            # 3. 检查变量命名
            for match in RE_VAR.finditer(line):
                name = match.group(1)
                # 排除 _ 通配符
                if name == "_": continue
                if not is_snake_case(name):
                    errors.append((i, f"变量名 '{name}' 应该使用 snake_case"))

            # 4. 检查结构体/枚举命名
            for match in RE_STRUCT.finditer(line):
                name = match.group(1)
                if not is_pascal_case(name):
                    errors.append((i, f"结构体名 '{name}' 应该使用 PascalCase"))
            
            for match in RE_ENUM.finditer(line):
                name = match.group(1)
                if not is_pascal_case(name):
                    errors.append((i, f"枚举名 '{name}' 应该使用 PascalCase"))

    return errors

def main():
    if not LENCYC_DIR.exists():
        print(f"Skipping lencyc meta check: {LENCYC_DIR} not found")
        return

    all_errors = {}
    for root, dirs, files in os.walk(LENCYC_DIR):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        for file in files:
            if file.endswith(".lcy"):
                path = Path(root) / file
                errors = check_file(path)
                if errors:
                    all_errors[path] = errors

    if all_errors:
        print("❌ Lencyc Naming Convention Errors:")
        for path, errors in all_errors.items():
            print(f"  {path}:")
            for line_num, msg in errors:
                print(f"    L{line_num}: {msg}")
        sys.exit(1)
    else:
        print("✅ Lencyc Meta Checks passed (Naming conventions, etc.)")

if __name__ == "__main__":
    main()
