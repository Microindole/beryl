from __future__ import annotations

from .models import MetaError, Token, TYPE_KEYWORDS, is_pascal_case, is_snake_case


class MetaChecker:
    def __init__(self, tokens: list[Token]) -> None:
        self.tokens = tokens
        self.index = 0
        self.errors: list[MetaError] = []

    def current(self) -> Token:
        return self.tokens[self.index]

    def previous(self) -> Token:
        return self.tokens[self.index - 1]

    def at_end(self) -> bool:
        return self.current().kind == "eof"

    def advance(self) -> Token:
        token = self.current()
        if not self.at_end():
            self.index += 1
        return token

    def check(self, value: str) -> bool:
        return self.current().value == value

    def match(self, value: str) -> bool:
        if self.check(value):
            self.advance()
            return True
        return False

    def peek(self, offset: int = 1) -> Token:
        idx = min(self.index + offset, len(self.tokens) - 1)
        return self.tokens[idx]

    def add_error(self, token: Token, message: str) -> None:
        self.errors.append(MetaError(token.line, message))

    def parse(self) -> list[MetaError]:
        while not self.at_end():
            if self.match("struct"):
                self.parse_struct_decl()
                continue
            if self.match("enum"):
                self.parse_enum_decl()
                continue
            if self.match("trait"):
                self.parse_trait_decl()
                continue
            if self.match("impl"):
                self.parse_impl_decl()
                continue
            if self.match("var") or self.match("const"):
                self.parse_var_like(self.previous())
                continue
            if self.looks_like_function_decl():
                self.parse_function_decl()
                continue
            self.skip_token_or_block()
        return self.errors

    def skip_token_or_block(self) -> None:
        if self.match("{"):
            self.skip_block()
            return
        self.advance()

    def skip_block(self) -> None:
        depth = 1
        while not self.at_end() and depth > 0:
            if self.match("{"):
                depth += 1
                continue
            if self.match("}"):
                depth -= 1
                continue
            self.advance()

    def consume_identifier(self) -> Token | None:
        token = self.current()
        if token.kind == "identifier":
            self.advance()
            return token
        return None

    def check_snake(self, token: Token | None, kind: str) -> None:
        if token is None:
            return
        if token.value != "_" and not is_snake_case(token.value):
            self.add_error(token, f"{kind}名 '{token.value}' 应该使用 snake_case")

    def check_pascal(self, token: Token | None, kind: str) -> None:
        if token is None:
            return
        if not is_pascal_case(token.value):
            self.add_error(token, f"{kind}名 '{token.value}' 应该使用 PascalCase")

    def skip_generic_args(self) -> None:
        if not self.check("<"):
            return
        depth = 0
        while not self.at_end():
            token = self.advance()
            if token.value == "<":
                depth += 1
                continue
            if token.value == ">":
                depth -= 1
                if depth == 0:
                    break

    def skip_type_ref(self) -> None:
        if self.current().kind not in {"identifier", "keyword"}:
            return
        if self.current().kind == "keyword" and self.current().value not in TYPE_KEYWORDS:
            return
        self.advance()
        while True:
            if self.check("<"):
                self.skip_generic_args()
                continue
            if self.match("?") or self.match("!"):
                continue
            if self.match("."):
                if self.current().kind in {"identifier", "keyword"}:
                    self.advance()
                    continue
            break

    def looks_like_function_decl(self) -> bool:
        start = self.index
        token = self.current()
        if token.kind not in {"identifier", "keyword"}:
            return False
        if token.kind == "keyword" and token.value not in TYPE_KEYWORDS:
            return False
        self.skip_type_ref()
        name = self.current()
        ok = name.kind == "identifier" and self.peek().value == "("
        self.index = start
        return ok

    def parse_function_decl(self) -> None:
        self.skip_type_ref()
        name = self.consume_identifier()
        self.check_snake(name, "函数")
        if self.match("<"):
            self.index -= 1
            self.skip_generic_args()
        if not self.match("("):
            return
        self.parse_param_list()
        if self.match("{"):
            self.parse_block_contents(until="}")
            return
        self.match(";")

    def parse_param_list(self) -> None:
        while not self.at_end() and not self.check(")"):
            self.skip_type_ref()
            param = self.consume_identifier()
            self.check_snake(param, "参数")
            if not self.match(","):
                break
        self.match(")")

    def parse_block_contents(self, until: str) -> None:
        while not self.at_end() and not self.check(until):
            if self.match("var") or self.match("const"):
                self.parse_var_like(self.previous())
                continue
            if self.match("struct"):
                self.parse_struct_decl()
                continue
            if self.match("enum"):
                self.parse_enum_decl()
                continue
            if self.match("trait"):
                self.parse_trait_decl()
                continue
            if self.match("impl"):
                self.parse_impl_decl()
                continue
            if self.looks_like_function_decl():
                self.parse_function_decl()
                continue
            if self.match("{"):
                self.parse_block_contents(until="}")
                continue
            self.advance()
        self.match(until)

    def parse_var_like(self, keyword: Token) -> None:
        name = self.consume_identifier()
        if name is None:
            self.add_error(keyword, f"'{keyword.value}' 后缺少变量名")
            return
        self.check_snake(name, "变量")

    def parse_struct_decl(self) -> None:
        name = self.consume_identifier()
        self.check_pascal(name, "结构体")
        if self.match("<"):
            self.index -= 1
            self.skip_generic_args()
        if not self.match("{"):
            return
        while not self.at_end() and not self.check("}"):
            if self.current().kind in {"identifier", "keyword"}:
                self.skip_type_ref()
                field = self.consume_identifier()
                self.check_snake(field, "字段")
                self.match(",")
                self.match(";")
                continue
            self.advance()
        self.match("}")

    def parse_enum_decl(self) -> None:
        name = self.consume_identifier()
        self.check_pascal(name, "枚举")
        if self.match("<"):
            self.index -= 1
            self.skip_generic_args()
        if not self.match("{"):
            return
        while not self.at_end() and not self.check("}"):
            variant = self.consume_identifier()
            self.check_pascal(variant, "枚举变体")
            if self.match("("):
                self.parse_enum_payload_types()
            self.match(",")
        self.match("}")

    def parse_enum_payload_types(self) -> None:
        while not self.at_end() and not self.check(")"):
            self.skip_type_ref()
            if not self.match(","):
                break
        self.match(")")

    def parse_trait_decl(self) -> None:
        name = self.consume_identifier()
        self.check_pascal(name, "Trait")
        if self.match("<"):
            self.index -= 1
            self.skip_generic_args()
        if not self.match("{"):
            return
        self.parse_block_contents(until="}")

    def parse_impl_decl(self) -> None:
        if self.match("<"):
            self.index -= 1
            self.skip_generic_args()
        if self.current().kind in {"identifier", "keyword"}:
            self.skip_type_ref()
        if self.match("for") and self.current().kind in {"identifier", "keyword"}:
            self.skip_type_ref()
        if not self.match("{"):
            return
        self.parse_block_contents(until="}")
