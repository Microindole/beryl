---
name: syntax
description: 词法与语法分析模块 (lency_syntax) 技术手册。
---

# Lency Syntax - 词法与语法分析

## 1. 核心职责
- **Lexing (词法)**: 使用 `logos` 将源码转换为 Token 流。
- **Parsing (语法)**: 使用 `chumsky` 处理复杂表达式，或手写递归下降解析声明/语句。
- **AST**: 定义于 `src/ast/`，所有节点必须包含 `Span`。

## 2. 关键文件 & 路径
| 功能 | 路径 | 关键点 |
|------|------|------|
| **Token 定义** | `src/lexer/mod.rs` | 关键词与符号映射。 |
| **AST 树** | `src/ast/mod.rs` | 包含 `Program`, `Decl`, `Stmt`, `Expr`。 |
| **解析逻辑** | `src/parser/` | 模块化解析（decl, stmt, expr 分开）。 |

## 3. 开发准则
- **Span 传递**: 必须在解析早期记录 `Span`，报错时定位。
- **递归支持**: 表达式解析注意结合性与优先级。
- **错误恢复**: 尽量捕获错误并继续解析，以收集更多诊断信息。

## 4. 避坑指南 (Pitfalls)
- **递归溢出**: Parser 深度嵌套时注意栈大小，复杂表达式优先用 `chumsky`。
- **Span 丢失**: 变换 AST 节点时若不显式传递 `Span`，会导致后续诊断显示“Unknown location”。
