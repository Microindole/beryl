# String \u683c\u5f0f\u5316\u529f\u80fd - Walkthrough

## \u5b9e\u73b0\u6982\u8981

\u65b0\u589e `format(string, Vec<string>) -> string` \u5185\u7f6e\u51fd\u6570\uff0c\u652f\u6301 `{}` \u5360\u4f4d\u7b26\u987a\u5e8f\u66ff\u6362\uff0c\u5b8c\u6210 Sprint 15 \u6700\u540e\u4e00\u9879\u4efb\u52a1\u3002

```lency
var args = vec!["world"]
var result = format("hello {}!", args)
// result == "hello world!"
```

## \u4fee\u6539\u6587\u4ef6 (11\u4e2a + 1\u4e2a\u65b0\u589e)

| \u5c42 | \u6587\u4ef6 | \u6539\u52a8 |
|---|---|---|
| Lexer | [lexer.rs](file:///home/indolyn/beryl/crates/lency_syntax/src/lexer.rs) | +`Token::Format` |
| AST | [expr.rs](file:///home/indolyn/beryl/crates/lency_syntax/src/ast/expr.rs) | +`ExprKind::Format(Box<Expr>, Box<Expr>)` |
| Parser | [intrinsics.rs](file:///home/indolyn/beryl/crates/lency_syntax/src/parser/expr/intrinsics.rs) | +format \u89e3\u6790\u5668 |
| Sema | [resolver/expr.rs](file:///home/indolyn/beryl/crates/lency_sema/src/resolver/expr.rs) | +\u53d8\u91cf\u89e3\u6790 |
| Sema | [type_infer/mod.rs](file:///home/indolyn/beryl/crates/lency_sema/src/type_infer/mod.rs) | +intrinsic \u5206\u53d1 |
| Sema | [type_infer/intrinsics.rs](file:///home/indolyn/beryl/crates/lency_sema/src/type_infer/intrinsics.rs) | +\u7c7b\u578b\u68c0\u67e5: template=string, args=Vec\u003cstring\u003e |
| Monomorph | [specializer/expr.rs](file:///home/indolyn/beryl/crates/lency_monomorph/src/specializer/expr.rs) | +\u5355\u6001\u5316\u900f\u4f20 |
| Monomorph | [collector.rs](file:///home/indolyn/beryl/crates/lency_monomorph/src/collector.rs) | +\u6cdb\u578b\u6536\u96c6 |
| Runtime | [string.rs](file:///home/indolyn/beryl/crates/lency_runtime/src/string.rs) | +`lency_string_format` FFI + \u5355\u5143\u6d4b\u8bd5 |
| Codegen | [string_ops.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/expr/string_ops.rs) | +`gen_format` \u51fd\u6570 |
| Codegen | [mod.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/expr/mod.rs) | +\u8def\u7531\u5206\u53d1 |
| Test | [string_format.lcy](file:///home/indolyn/beryl/tests/integration/stdlib/string_format.lcy) | \u65b0\u589e\u96c6\u6210\u6d4b\u8bd5 |

## \u9a8c\u8bc1\u7ed3\u679c

`./scripts/run_checks.sh` \u5168\u90e8\u901a\u8fc7\uff1a
- Format check: PASS
- Clippy: PASS
- Unit tests: PASS (\u542b `test_string_format`)
- Lcy tests: **63 passed**, 5 expected failures, 0 unexpected
- File size / Code quality / Editor: PASS
