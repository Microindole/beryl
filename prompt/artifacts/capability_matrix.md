# Lency 双链路能力矩阵（Phase 0 基线）

更新时间：2026-03-07
状态枚举：`NotStarted` / `InProgress` / `Done`

## 1. 语法（Syntax）
| 能力项 | Rust 主链路 | Lency 自举链路 | 差距级别 |
|---|---|---|---|
| 基础表达式与控制流（if/while/for） | Done | Done | 低 |
| 函数声明与调用 | Done | InProgress | 中 |
| struct/impl 最小声明 | Done | InProgress | 中 |
| const 声明 | Done | Done | 低 |
| import 声明 | Done | Done | 低 |
| extern 声明 | Done | Done | 低 |
| enum 声明 | Done | InProgress（unit variant 子集） | 中 |
| match/case | Done | NotStarted | 高 |
| null / nullable 语法入口 | Done | NotStarted | 高 |
| trait 语法 | Done | NotStarted | 高 |
| 泛型语法（声明 + 调用侧） | Done | NotStarted | 高 |

## 2. 语义（Semantic）
| 能力项 | Rust 主链路 | Lency 自举链路 | 差距级别 |
|---|---|---|---|
| 名称解析（undefined/duplicate/scope） | Done | Done | 低 |
| 基础类型一致性（int/bool/string/float） | Done | Done | 低 |
| 函数签名与调用 arity 校验 | Done | Done | 低 |
| return 合法性校验 | Done | Done | 低 |
| struct 字段重名/未知类型校验 | Done | Done | 低 |
| impl 目标类型与方法去重校验 | Done | Done | 低 |
| import 符号绑定 | Done | InProgress（仅 alias 绑定） | 中 |
| extern 签名绑定 | Done | Done | 低 |
| nullable/result 语义 | Done | NotStarted | 高 |
| enum + match 语义一致性 | Done | NotStarted | 高 |
| 泛型约束与实例化语义 | Done | NotStarted | 高 |
| trait 约束与实现匹配 | Done | NotStarted | 高 |

## 3. 后端（Backend）
| 能力项 | Rust 主链路 | Lency 自举链路 | 差距级别 |
|---|---|---|---|
| AST -> LLVM IR 主路径 | Done | NotStarted | 高 |
| AST -> LIR 发射 | NotStarted | Done | 中 |
| `.lir -> LLVM -> executable` | Done | InProgress（借 Rust backend） | 中 |
| call/member lowering 完整性 | InProgress（存在 FIXME） | NotStarted | 高 |
| runtime builtin 类型对齐 | InProgress | NotStarted | 高 |

## 4. 工具链（Tooling）
| 能力项 | Rust 主链路 | Lency 自举链路 | 差距级别 |
|---|---|---|---|
| `xtask` 双检查闭环 | Done | Done | 低 |
| CLI 基础构建与运行 | Done | InProgress | 中 |
| 自举 one-step build/run 回归 | Done | Done | 低 |
| 文档与实现一致性门禁 | InProgress | InProgress | 中 |

## 5. 阶段优先级映射
1. Phase 1（语法补齐第一批）：`const/import/extern/enum/match/null`
2. Phase 2（语义补齐第一批）：`import/extern/nullable/result/enum+match`
3. Phase 3（泛型 + trait）：声明、实例化、约束与匹配
4. Phase 4（后端补齐）：LIR 指令覆盖与 lowering 完整性
5. Phase 5（可用性冲刺）：样例通过率与文档一致性

## 6. 本周执行落点
- 已启动：Phase 0（本文件）
- 已完成：Phase 1 子项 1 `const`（语法 + parser 回归 + sema 最小约束）
- 已完成：Phase 1 子项 2 `import/extern`（语法 + parser 回归 + resolver 绑定骨架）
- 已完成（部分）：Phase 1 子项 3 `enum`（unit variant + parser/sema 最小回归）
- 下一项：Phase 1 子项 3 剩余 `match` + `enum payload`（最小语法 + AST + parser 回归）
