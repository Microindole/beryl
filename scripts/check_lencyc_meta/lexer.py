from __future__ import annotations

from .models import KEYWORDS, Token, is_identifier_part, is_identifier_start


def tokenize(source: str) -> list[Token]:
    tokens: list[Token] = []
    i = 0
    line = 1
    col = 1
    length = len(source)
    # FIXME: Lency 当前未支持 /* ... */ 块注释；若未来语言扩展到块注释，这里必须同步扩展。

    def advance(n: int = 1) -> str:
        nonlocal i, line, col
        chunk = source[i : i + n]
        for ch in chunk:
            if ch == "\n":
                line += 1
                col = 1
            else:
                col += 1
        i += n
        return chunk

    while i < length:
        ch = source[i]

        if ch in " \t\r":
            advance()
            continue
        if ch == "\n":
            advance()
            continue

        if ch == "/" and i + 1 < length and source[i + 1] == "/":
            while i < length and source[i] != "\n":
                advance()
            continue

        if ch == '"':
            start_line, start_col = line, col
            advance()
            escaped = False
            while i < length:
                cur = source[i]
                if escaped:
                    advance()
                    escaped = False
                    continue
                if cur == "\\":
                    advance()
                    escaped = True
                    continue
                if cur == '"':
                    advance()
                    break
                advance()
            tokens.append(Token("string", '""', start_line, start_col))
            continue

        if ch == "'":
            start_line, start_col = line, col
            advance()
            escaped = False
            while i < length:
                cur = source[i]
                if escaped:
                    advance()
                    escaped = False
                    continue
                if cur == "\\":
                    advance()
                    escaped = True
                    continue
                if cur == "'":
                    advance()
                    break
                advance()
            tokens.append(Token("char", "''", start_line, start_col))
            continue

        if ch.isdigit():
            start_line, start_col = line, col
            start = i
            advance()
            while i < length and (source[i].isalnum() or source[i] in "._"):
                advance()
            tokens.append(Token("number", source[start:i], start_line, start_col))
            continue

        if is_identifier_start(ch):
            start_line, start_col = line, col
            start = i
            advance()
            while i < length and is_identifier_part(source[i]):
                advance()
            value = source[start:i]
            kind = "keyword" if value in KEYWORDS else "identifier"
            tokens.append(Token(kind, value, start_line, start_col))
            continue

        start_line, start_col = line, col
        two_char = source[i : i + 2]
        if two_char in {"->", "=>", "==", "!=", "<=", ">=", "&&", "||"}:
            advance(2)
            tokens.append(Token("symbol", two_char, start_line, start_col))
            continue
        advance()
        tokens.append(Token("symbol", ch, start_line, start_col))

    tokens.append(Token("eof", "", line, col))
    return tokens
