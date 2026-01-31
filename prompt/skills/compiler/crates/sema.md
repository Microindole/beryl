---
name: sema
description: 语义分析与类型检查模块 (lency_sema) 技术手册。
---

# Lency Sema - 语义分析

## 1. 核心流水线 (The Three Passes)
1. **Resolver**: 遍历 AST，收集全局定义（Struct, Enum, Function），构建符号表。
2. **TypeChecker**: 递归推导表达式类型，执行类型匹配检查。
3. **NullSafety**: 专用 Pass，检查非空赋值与可空解包状态。

## 2. 核心机制
- **ScopeStack**: 管理嵌套作用域，支持符号查找。
- **TypeRegistry**: 集中管理基础类型、用户定义类型及 Trait。
- **BinaryOpRegistry**: 注册所有运算符的合法类型组合及其结果。

## 3. 开发准则
- **显式错误**: 发现语义矛盾立即产出 `SemanticError`。
- **延迟单态化**: 记录泛型符号，但不在此处进行实例化。
- **代码导航**: `analyze` 函数是集成入口。

## 4. 避坑指南 (Pitfalls)
- **作用域泄漏**: 进入闭包或循环时必须正确 `push_scope` / `pop_scope`。
- **重复报错**: 同一个表达式错误只需在 `TypeChecker` 报错一次，避免日志洪泛。

---
[语义入口](../../../../crates/lency_sema/src/lib.rs) | [符号定义](../../../../crates/lency_sema/src/symbol/mod.rs)
