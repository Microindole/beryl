# Lency 项目上下文入口

## 0. 最高准则
- 语言与设计哲学：`assets/Lency.txt`、`assets/design_spec.md`（冲突时以这两个文件为准）。
- 本文件只做“地图与职责”，不再记录逐条开发日志。

## 1. 目录地图（先看这里）
- `crates/`：Rust 编译器主实现（稳定链路、CI 主体）。
- `lencyc/`：Lency 自举编译器（当前重点：Lexer/Parser/Sema 逐步对齐）。
- `lib/`：标准库源码（Rust/Lency 两侧都会受影响）。
- `tests/integration/`：Rust 侧集成测试。
- `tests/example/`：Lency 侧示例/实验测试。
- `scripts/run_checks.sh`：Rust 侧固定检查入口（不接收参数）。
- `scripts/run_lency_checks.sh`：Lency 侧固定检查入口（不接收参数）。
- `prompt/sprint/status.md`：当前 sprint 状态与里程碑。
- `prompt/artifacts/`：任务记录（task / plan / walkthrough）。
- `docs/`：用户文档（语言行为变化时必须同步）。

## 2. 协作与记录规则
- 进度状态：只更新 `prompt/sprint/status.md`。
- 任务过程：写入 `prompt/artifacts/` 对应文件。
- 架构变化：必要时补充到本文件“长期约定”，不要写流水账。
- Lency 语法检查约定：`run_lency_checks.sh` 会优先使用 `lencyc build --check-only` 对 `lencyc/driver/test_entry.lcy` 与 `lencyc/driver/main.lcy` 做入口级语法检查；若未来该参数缺失，脚本才会回退为跳过并由完整 build 覆盖。
- 每次改动结束必须运行：
  - `./scripts/run_checks.sh`
  - `./scripts/run_lency_checks.sh`

## 3. CI 触发约定（摘要）
- CI 先按路径判定改动作用域，再触发对应 job。
- Rust 作用域：`crates/**`、`tests/integration/**`、以及共享项（如 `lib/**`、部分脚本/workflow）。
- Lency 作用域：`lencyc/**`、`tests/example/**`、以及共享项（如 `lib/**`、部分脚本/workflow）。
- `macos-check` 当前仅跟随 Rust 作用域触发（main 分支或手动触发）。

## 4. 当前工作焦点（自举）
- 已完成：Parser/AST 模块化拆分（`lencyc/syntax/{parser,ast}/...`）。
- 已支持：`break/continue` 语句及循环外非法位置约束（parser 直接报错）。
- 已支持：C 风格 `for` 语句基础解析（当前通过 parser 反糖到 `while`）。
- 语义修正：`for` 反糖路径下，`continue` 已确保先执行 `increment`（且不影响嵌套循环）。
- 解析边界：`for` 当前支持 `var` 或表达式初始化（如 `for var i = ...;` / `for i = ...;`）。
- 表达式能力：parser 已支持 `call` 与 `member` 链（`foo(a,b)`、`obj.method()`），并支持字符串字面量（`"text"`）。
- 数字字面量：lexer 已支持 `int/float/scientific`（如 `1`、`3.14`、`1.23e-4`、`9E+2`）。
- 字符串/字符字面量：lexer 已支持字符串转义扫描（如 `\"`、`\\n`）与字符字面量（如 `'a'`、`'\\n'`）。
- Lency 自举 TODO 状态：`lencyc/` 目录内 `TODO` 已清零；当前剩余 TODO 仅在 `lib/std` 与 Rust 编译器路径。
- 自举语义骨架：已添加最小 `name resolution`（变量定义/引用检查）并接入 `test_entry` 烟雾验证。
- 语义测试：`test_entry` 已补 resolver 负例（undefined/duplicate），不再只测正例。
- 回归结构化：测试样例已抽离到 `lencyc/driver/test_cases.lcy`，`test_entry` 改为用例编排执行。
- 最小完整链路：`lencyc/driver/main.lcy` 已串联 `Read -> Lex -> Parse -> Resolve -> Emit(AST 文本)`，默认输入 `lencyc/driver/pipeline_sample.lcy`。
- 后端演进：`lencyc` 已增加最小 LIR 文本发射（`--emit-lir`），用于对接后续 Rust LLVM backend；当前默认 emit 仍为 AST 文本以保持自举稳定。
- 回归约束：`run_lency_checks.sh` 已纳入 `tests/example/lencyc_lir_*.lcy` 用例，固定校验自举 `--emit-lir` 输出结构。
- Rust 后端进展：`crates/lency_cli` 已支持最小 `.lir -> LLVM IR -> executable` 构建路径（`lencyc build file.lir`），并接入 Lency 侧脚本的端到端冒烟。
- Rust `.lir` backend 能力增量：已支持最小外部函数调用 lowering（`call %foo(...)` -> LLVM `declare i64 @foo(...)` + `call`）。
- LIR 回归样例扩展：`tests/example` 新增 `lencyc_lir_unary_logic.lcy` 与 `lencyc_lir_break_continue.lcy`，已纳入 `run_lency_checks.sh`。
- 可用性打通：新增 `scripts/lency_selfhost_build.sh`，提供 `.lcy -> self-host emit-lir -> Rust backend build` 的一键构建路径，并已接入 Lency 检查脚本闭环验证。
- 运行闭环：新增 `scripts/lency_selfhost_run.sh`，提供 `.lcy -> self-host build -> run` 一键运行路径（支持参数透传与期望退出码校验），并已接入 Lency 检查脚本。
- 当前策略：按语法特性小步增量推进，每次增量后立刻跑 Lency 检查，避免回归。
- 下一阶段：在保持可运行的前提下逐步补齐语句与语义能力。
