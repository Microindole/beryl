#!/usr/bin/env python3
"""
目录同层文件密度检查（启发式）

目标：提示目录污染风险。
- warning: 同层文件数 >= 9
- error:   同层文件数 >= 20
"""

import argparse
import os
import sys
from pathlib import Path

EXCLUDE_DIRS = {".git", "target", "node_modules", ".gemini", "__pycache__", ".idea", ".vscode"}
SOURCE_EXTS = {".rs", ".lcy", ".py", ".sh", ".md", ".txt"}
WARN_THRESHOLD = 9
ERROR_THRESHOLD = 20


def roots_for_scope(scope: str):
    if scope == "rust":
        return [Path("crates"), Path("lib"), Path("tests/integration")]
    if scope == "lency":
        return [Path("lencyc"), Path("lib"), Path("tests/example"), Path("xtask")]
    return [Path(".")]


def scan(root: Path):
    warnings = []
    errors = []
    for current, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        p = Path(current)
        candidates = [f for f in files if Path(f).suffix in SOURCE_EXTS]
        count = len(candidates)
        if count >= ERROR_THRESHOLD:
            errors.append((p, count))
        elif count >= WARN_THRESHOLD:
            warnings.append((p, count))
    return warnings, errors


def main():
    parser = argparse.ArgumentParser(description="检查目录同层文件密度")
    parser.add_argument("--scope", choices=["all", "rust", "lency"], default="all")
    args = parser.parse_args()

    project_root = Path(__file__).parent.parent.resolve()
    os.chdir(project_root)

    roots = roots_for_scope(args.scope)
    all_warnings = []
    all_errors = []
    for r in roots:
        if not r.exists():
            continue
        w, e = scan(r)
        all_warnings.extend(w)
        all_errors.extend(e)

    print(f"🔍 扫描目录同层文件密度 (scope={args.scope})")
    print(f"   警告阈值: {WARN_THRESHOLD}")
    print(f"   错误阈值: {ERROR_THRESHOLD}")

    if all_errors:
        print("\n❌ 错误：以下目录同层文件数过高（建议立即下沉分层）")
        for path, count in sorted(all_errors, key=lambda x: x[1], reverse=True):
            print(f"   {path.as_posix()}: {count}")

    if all_warnings:
        print("\n⚠️  警告：以下目录接近污染阈值（建议评估下沉）")
        for path, count in sorted(all_warnings, key=lambda x: x[1], reverse=True):
            print(f"   {path.as_posix()}: {count}")

    if not all_errors and not all_warnings:
        print("\n✅ 未发现目录密度问题")

    sys.exit(1 if all_errors else 0)


if __name__ == "__main__":
    main()
