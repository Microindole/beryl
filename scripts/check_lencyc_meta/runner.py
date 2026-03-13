from __future__ import annotations

import os
import sys
from pathlib import Path

from .checker import MetaChecker
from .lexer import tokenize
from .models import MetaError

LENCYC_DIR = Path("lencyc")
EXCLUDE_DIRS = {".git", "target", "node_modules", ".gemini", "__pycache__"}


def has_header_comment(file_path: Path) -> bool:
    with open(file_path, "r", encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if not stripped:
                continue
            return stripped.startswith("//")
    return True


def check_file(file_path: Path) -> list[MetaError]:
    errors: list[MetaError] = []
    if not has_header_comment(file_path):
        errors.append(MetaError(1, "文件缺少头注释（首个非空行应为 // 注释）"))

    with open(file_path, "r", encoding="utf-8") as f:
        source = f.read()

    try:
        tokens = tokenize(source)
    except Exception as exc:
        errors.append(MetaError(1, f"词法扫描失败: {exc}"))
        return errors

    checker = MetaChecker(tokens)
    errors.extend(checker.parse())
    return errors


def collect_errors() -> dict[Path, list[MetaError]]:
    all_errors: dict[Path, list[MetaError]] = {}
    for root, dirs, files in os.walk(LENCYC_DIR):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        for file_name in files:
            if not file_name.endswith(".lcy"):
                continue
            path = Path(root) / file_name
            errors = check_file(path)
            if errors:
                all_errors[path] = errors
    return all_errors


def main() -> None:
    if not LENCYC_DIR.exists():
        print(f"Skipping lencyc meta check: {LENCYC_DIR} not found")
        return

    all_errors = collect_errors()
    if all_errors:
        print("ERROR: Lencyc Meta Check Errors:")
        for path, errors in sorted(all_errors.items(), key=lambda item: item[0].as_posix()):
            print(f"  {path}:")
            for error in errors:
                print(f"    L{error.line}: {error.message}")
        sys.exit(1)

    print("OK: Lencyc Meta Checks passed (structured naming/header checks)")
