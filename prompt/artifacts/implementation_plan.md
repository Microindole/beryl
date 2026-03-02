# Sprint 17 Implementation Plan (Parser)

## 目标
以“最小增量 + 立即验证”的节奏推进 `lencyc` 自举 Parser，避免一次性大改导致回归。

## 本轮已完成 (2026-03-02)
1. 在 `lencyc/syntax/ast.lcy` 新增 `STMT_RETURN` 与 `make_stmt_return`。
2. 在 `lencyc/syntax/parser.lcy` 新增 `return_statement()` 并接入 `statement()` 分发。
3. 在 `lencyc/driver/test_entry.lcy` 加入 `return` 语句样例，验证解析路径。
4. 在 `lencyc/syntax/ast.lcy` 新增 `expr_to_string` / `stmt_to_string` AST Printer。
5. 在 `lencyc/driver/test_entry.lcy` 输出 AST 文本，提升可观测性。
6. 运行 `./scripts/run_lency_checks.sh`，通过。

## 下一步 (按优先级)
1. 声明解析扩展
   - 支持 `func`/类型声明的最小骨架（若暂未完成，保留 TODO 注释）。
2. 词法补强
   - 补齐字符串字面量与浮点字面量。
   - 清理已完成但过期的 TODO 注释。

## 质量门禁
每次新增一个语法点，立即执行：
1. `./scripts/run_lency_checks.sh`
2. 在阶段性提交前执行 `./scripts/run_checks.sh`

## 当前阻塞/技术债
1. `return` 无值场景仍使用哑元表达式承载
   - TODO: 支持 void-return 的专用 AST 节点，避免哑元表达式。
2. Parser 报错仍为本地 `print`，尚未接入统一 Reporter。
