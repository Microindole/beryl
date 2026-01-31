---
name: codegen
description: LLVM 代码生成模块 (lency_codegen) 技术手册。
---

# Lency Codegen - 代码生成

## 1. 技术栈
- **Inkwell**: LLVM 的类型安全 Rust 包装器。
- **LLVM Optimizer**: 调用默认优化流水线提升产出质量。

## 2. 生成流程 (ModuleGenerator)
- **Types**: `Lency Type` -> `LLVM Type` (i64, ptr, struct)。
- **Function**: 生成 `__lency_<name>` 并在必要时包装为标准的 `main`。
- **Expr/Stmt**: 递归生成指令。加载 (Load) 与 存储 (Store) 需严格匹配。

## 3. 实战范式
- **Intrinsic 获取**: 使用 `ctx.get_intrinsic("name")` 调用运行时函数。
- **符号查找**: 总是优先从单态化后的符号表中解析地址。
- **空安全导出**: Codegen 假设 Sema 已放行，不在此处做空检查。

## 4. 避坑指南 (Pitfalls)
- **i32 vs i64**: Lency 内部 `int` 默认映射到 LLVM `i64`，混用会导致指令报错。
- **对齐问题**: 生成 Struct 内存布局时必须遵循 C ABI 对齐原则，否则 FFI 会崩溃。

---
[Codegen 上下文](../../../../crates/lency_codegen/src/context.rs) | [表达式生成](../../../../crates/lency_codegen/src/expr/mod.rs)
