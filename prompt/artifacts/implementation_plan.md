# String 格式化功能实现计划

实现 `format(template, args)` 内置函数，支持 `{}` 占位符替换，完成 Sprint 15 最后一项核心任务。

## 设计决策

> [!IMPORTANT]
> **方案选择**：`plan_15.md` 中原定了方案A (`format`) 和方案B (`concat` 系列)两种路线。
> 鉴于 `+` 运算符已通过 `string_ops::concat` 支持字符串拼接，方案B（`concat` 函数）价值有限。
> 推荐直接实现**方案A: `format(string, Vec<string>)`**，这对自举编译器的错误消息格式化至关重要。

**函数签名**：
```lency
// format("Error at line {}: {}", args) -> "Error at line 42: unexpected token"
// args 是 Vec<string>，每个 {} 顺序替换为 args 中对应元素
string format(string template, Vec<string> args)
```

**为什么不用变参**：Lency 目前不支持变参函数，使用 `Vec<string>` 是最简洁的方式，与语言哲学一致。

## Proposed Changes

### Lexer (lency_syntax)

#### [MODIFY] [lexer.rs](file:///home/indolyn/beryl/crates/lency_syntax/src/lexer.rs)

在 `Token::Panic` 之后添加 `Token::Format`：
```diff
 #[token("panic")]
 Panic,
+#[token("format")]
+Format,
```

---

### AST (lency_syntax)

#### [MODIFY] [expr.rs](file:///home/indolyn/beryl/crates/lency_syntax/src/ast/expr.rs)

在 `ExprKind::Panic` 之后添加：
```diff
 Panic(Box<Expr>),
+// format("template {}", args_vec) -> string
+Format(Box<Expr>, Box<Expr>),
```

---

### Parser (lency_syntax)

#### [MODIFY] [intrinsics.rs](file:///home/indolyn/beryl/crates/lency_syntax/src/parser/expr/intrinsics.rs)

添加 `format(template, args)` 解析器，模式与 `split` / `join` 完全一致（两个参数，逗号分隔）：
```rust
let format_expr = just(Token::Format)
    .ignore_then(
        expr.clone()
            .then_ignore(just(Token::Comma))
            .then(expr.clone())
            .delimited_by(just(Token::LParen), just(Token::RParen)),
    )
    .map_with_span(|(template, args), span| Expr {
        kind: ExprKind::Format(Box::new(template), Box::new(args)),
        span,
    });
```

在组合器链末尾追加 `.or(format_expr)`。

---

### 语义分析 (lency_sema)

#### [MODIFY] [resolver/expr.rs](file:///home/indolyn/beryl/crates/lency_sema/src/resolver/expr.rs)

在 `ExprKind::Panic` 处理分支附近添加：
```rust
ExprKind::Format(template, args) => {
    self.resolve_expr(template);
    self.resolve_expr(args);
}
```

#### [MODIFY] [type_infer/mod.rs](file:///home/indolyn/beryl/crates/lency_sema/src/type_infer/mod.rs)

在 intrinsic 分发列表中添加 `ExprKind::Format(_, _)`。

#### [MODIFY] [type_infer/intrinsics.rs](file:///home/indolyn/beryl/crates/lency_sema/src/type_infer/intrinsics.rs)

添加类型检查：template 必须是 `string`，args 必须是 `Vec<string>`，返回 `string`。

---

### 单态化 (lency_monomorph)

#### [MODIFY] [specializer/expr.rs](file:///home/indolyn/beryl/crates/lency_monomorph/src/specializer/expr.rs)

在 `ExprKind::Panic` 之后添加：
```rust
ExprKind::Format(template, args) => ExprKind::Format(
    Box::new(spec.specialize_expr(template)),
    Box::new(spec.specialize_expr(args)),
),
```

#### [MODIFY] [collector.rs](file:///home/indolyn/beryl/crates/lency_monomorph/src/collector.rs)

在 `ExprKind::Panic` 之后添加：
```rust
ExprKind::Format(template, args) => {
    self.collect_expr(template);
    self.collect_expr(args);
}
```

---

### Runtime (lency_runtime)

#### [MODIFY] [string.rs](file:///home/indolyn/beryl/crates/lency_runtime/src/string.rs)

添加 `lency_string_format` FFI 函数：
```rust
/// format("template {}", vec_of_strings) -> "template value"
/// 将模板中的 {} 占位符按顺序替换为 Vec 中的字符串
#[no_mangle]
pub unsafe extern "C" fn lency_string_format(
    template_ptr: *const c_char,
    vec_ptr: *const LencyVec,
) -> *mut c_char
```

实现逻辑：遍历 template 字符串，遇到 `{}` 时从 vec 中取下一个元素替换，结果 malloc 返回。

添加对应的单元测试。

---

### Codegen (lency_codegen)

#### [MODIFY] [string_ops.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/expr/string_ops.rs)

添加 `gen_format` 函数，声明并调用 `lency_string_format` FFI 函数，模式与 `gen_join` 完全一致。

#### [MODIFY] [mod.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/expr/mod.rs)

在 `ExprKind::Panic` 之前添加路由：
```rust
ExprKind::Format(template, args) => string_ops::gen_format(ctx, locals, template, args),
```

---

### 集成测试

#### [NEW] [string_format.lcy](file:///home/indolyn/beryl/tests/integration/stdlib/string_format.lcy)

测试用例：
1. 基础替换：`format("hello {}", args)` 其中 args 包含 `"world"`
2. 多参数替换：`format("{} + {} = {}", args)` 
3. 无占位符：`format("no placeholders", empty_args)`
4. 空模板 / 空参数边界情况

---

## Verification Plan

### Automated Tests

1. **Rust 单元测试** (runtime 层)
   ```bash
   cd /home/indolyn/beryl && cargo test -p lency_runtime -- string::tests
   ```

2. **全量 Rust 测试** (确保无回归)
   ```bash
   cd /home/indolyn/beryl && cargo test
   ```

3. **集成测试** (.lcy 文件)
   ```bash
   cd /home/indolyn/beryl && bash scripts/run_lcy_tests.sh
   ```

4. **完整检查** (fmt + clippy + test + lcy + file size)
   ```bash
   cd /home/indolyn/beryl && bash scripts/run_checks.sh
   ```

### 修改文件总计

| 层 | 文件 | 改动类型 |
|---|---|---|
| Lexer | `lexer.rs` | +2行 |
| AST | `expr.rs` | +2行 |
| Parser | `intrinsics.rs` | +15行 |
| Sema | `resolver/expr.rs`, `type_infer/mod.rs`, `type_infer/intrinsics.rs` | ~+20行 |
| Monomorph | `specializer/expr.rs`, `collector.rs` | +8行 |
| Codegen | `string_ops.rs`, `mod.rs` | +40行 |
| Runtime | `string.rs` | +60行 |
| Test | `string_format.lcy` | 新文件 |

**总计**：~147行新增，0行删除，涉及11个文件。
