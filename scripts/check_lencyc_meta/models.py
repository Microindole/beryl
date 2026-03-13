from __future__ import annotations

import re
from dataclasses import dataclass

KEYWORDS = {
    "if",
    "else",
    "var",
    "const",
    "import",
    "extern",
    "enum",
    "trait",
    "match",
    "null",
    "true",
    "false",
    "while",
    "struct",
    "impl",
    "return",
    "void",
    "int",
    "string",
    "bool",
    "float",
    "break",
    "continue",
    "for",
}

TYPE_KEYWORDS = {"void", "int", "string", "bool", "float"}


@dataclass(frozen=True)
class Token:
    kind: str
    value: str
    line: int
    column: int


@dataclass(frozen=True)
class MetaError:
    line: int
    message: str


def is_snake_case(name: str) -> bool:
    if not name:
        return True
    if name.isupper():
        return True
    return bool(re.match(r"^[a-z][a-z0-9_]*$", name))


def is_pascal_case(name: str) -> bool:
    if not name:
        return True
    return bool(re.match(r"^[A-Z][a-zA-Z0-9]*$", name))


def is_identifier_start(ch: str) -> bool:
    return ch.isalpha() or ch == "_"


def is_identifier_part(ch: str) -> bool:
    return ch.isalnum() or ch == "_"
