---
name: compiler
description: 全链路开发与调试手册。
---

# 编译器工程师 (Compiler) - 导航地图

## 1. 核心管道与专精指南
点击下方链接加载特定 Crate 的深度工程手册：

| 阶段 | Crate | 专精领域 | 详细指南 |
|------|-------|---------|---------|
| **1. 语法** | `lency_syntax` | AST, Lexer, Parser | [查看指南](./crates/syntax.md) |
| **2. 语义** | `lency_sema` | 类型推导, 符号表, 空安全 | [查看指南](./crates/sema.md) |
| **3. 实例化**| `lency_monomorph`| 泛型单态化, 符号混淆 | [查看指南](./crates/monomorph.md) |
| **4. 后端** | `lency_codegen` | LLVM IR, Inkwell, 布控 | [查看指南](./crates/codegen.md) |
| **5. 运行** | `lency_runtime` | C-FFI, 堆管理, Intrinsics | [查看指南](./crates/runtime.md) |

## 2. 跨 Crate 联动说明 (Dependency Map)
- **Sema -> Monomorph**: Sema 必须准确标记哪些泛型被实例化。
- **Monomorph -> Codegen**: Codegen 依赖生成的混淆符号进行函数跳转。
- **Codegen -> Runtime**: Codegen 必须知道 Runtime 导出的 `extern C` 函数名。

## 3. 快速调试实验室
- **语义自检**: `cargo run --bin lencyc -- check <file>`
- **产出生成器**: `cargo run --bin lencyc -- compile <file> --emit-llvm`
- **全量回归**: `./scripts/run_checks.sh --fast`

## 4. 核心菜谱库 (Recipe Bank)
| ID | 任务 | 操作路径 |
|----|------|---------|
| **A1** | **加新关键字** | `syntax/lexer` -> `syntax/parser/decl` -> `sema/resolver` |
| **A2** | **加内建方法** | `runtime` (C) -> `codegen/intrinsic` -> `sema/type_check` |
| **A3** | **改语法结构** | `syntax/ast` -> `syntax/parser` -> `sema` -> `codegen` |
| **A4** | **调试段错误** | `emit-llvm` -> `clang -g` -> `gdb` |

---
[诊断系统 (lency_diagnostics)](../../../crates/lency_diagnostics/src/lib.rs)
