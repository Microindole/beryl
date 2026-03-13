#!/usr/bin/env python3
"""Unit tests for structured lencyc meta checks."""

from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from scripts.check_lencyc_meta.checker import MetaChecker
from scripts.check_lencyc_meta.lexer import tokenize
from scripts.check_lencyc_meta.runner import check_file


def collect_messages(source: str) -> list[str]:
    checker = MetaChecker(tokenize(source))
    return [error.message for error in checker.parse()]


class LexerTests(unittest.TestCase):
    def test_tokenize_skips_line_comments_and_string_bodies(self) -> None:
        source = """// header
string render_name() {
    // TODO: ignored in comment
    var msg = "struct Bad { int nope }"
    return msg
}
"""
        tokens = tokenize(source)
        values = [token.value for token in tokens if token.kind != "eof"]
        self.assertIn("string", values)
        self.assertIn("render_name", values)
        self.assertIn('""', values)
        self.assertNotIn("Bad", values)


class CheckerTests(unittest.TestCase):
    def test_accepts_generic_function_and_impl_shapes(self) -> None:
        source = """// header
struct Box<T> {
    T value
}

impl<T> Box<T> {
    T get_value() {
        var local_name = 1
        return local_name
    }
}
"""
        self.assertEqual(collect_messages(source), [])

    def test_reports_naming_violations(self) -> None:
        source = """// header
struct bad_box {
    int Value
}

enum color_kind {
    red_light,
}

int BadFunc(int BadArg) {
    var WrongName = 1
    return WrongName
}
"""
        messages = collect_messages(source)
        self.assertTrue(any("结构体名 'bad_box'" in msg for msg in messages))
        self.assertTrue(any("字段名 'Value'" in msg for msg in messages))
        self.assertTrue(any("枚举名 'color_kind'" in msg for msg in messages))
        self.assertTrue(any("枚举变体名 'red_light'" in msg for msg in messages))
        self.assertTrue(any("函数名 'BadFunc'" in msg for msg in messages))
        self.assertTrue(any("参数名 'BadArg'" in msg for msg in messages))
        self.assertTrue(any("变量名 'WrongName'" in msg for msg in messages))


class RunnerTests(unittest.TestCase):
    def test_check_file_requires_header_comment(self) -> None:
        with tempfile.TemporaryDirectory() as tmp_dir:
            file_path = Path(tmp_dir) / "sample.lcy"
            file_path.write_text("int main() { return 0 }\n", encoding="utf-8")
            errors = check_file(file_path)
        self.assertTrue(any("文件缺少头注释" in error.message for error in errors))

    def test_check_file_reports_missing_var_name(self) -> None:
        with tempfile.TemporaryDirectory() as tmp_dir:
            file_path = Path(tmp_dir) / "sample.lcy"
            file_path.write_text("// header\nvar = 1\n", encoding="utf-8")
            errors = check_file(file_path)
        self.assertTrue(any("'var' 后缺少变量名" in error.message for error in errors))


if __name__ == "__main__":
    unittest.main()
