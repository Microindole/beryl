---
name: architect
description: 核心设计哲学与语法契约。
---

# 架构师 (Architect) - Lency 准则

## 1. 核心铁律
- **Crystal Clear**: 禁止隐式转换，意图必须字面可见。
- **Safety by Default**: 默认非空；`T?` 必须解包；禁止 `unwrap` 黑魔法。
- **Simplicity First**: 宁可冗长而清晰，不可精简而晦涩。拒绝元编程。

## 2. 语法契约 (Rules)
- **命名**: 类型 `PascalCase` | 函数/变量 `snake_case` | 常量 `SCREAMING_SNAKE_CASE`。
- **结构**: 强制 `{}` | 禁止分号 `;` | 条件不加 `()`。
- **逻辑**: 组合优于继承 | 显式优于隐式。

## 3. 决策矩阵 (Decision Matrix)
任何新特性/语法必须过三关：
1. **显式度**: 读者能否在不看文档的情况下猜出意图？(Must be > 80%)
2. **自举成本**: 用 Lency 实现该特性是否需要大量魔法？(Must be Low)
3. **标准化**: 是否符合 C 系直觉？(No Python/Rust styles)

## 4. 命名速查 (Cheat Sheet)
| 对象 | 格式 | 示例 |
|------|------|------|
| **泛型参数** | 单大写字母 | `T`, `E`, `K`, `V` |
| **内部辅助函数** | `_` 开头 | `_internal_init` |
| **转换方法** | `to_` / `as_` | `to_string`, `as_int` |
| **谓词/检查** | `is_` / `has_` | `is_empty`, `has_key` |

---
[详细规范](../../../assets/design_spec.md) | [构想原件](../../../assets/Lency.txt)
