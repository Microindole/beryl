# Editors 子上下文入口

## 0. 最高准则
- 设计哲学仍以 `assets/Lency.txt`、`assets/design_spec.md` 为准。
- 本文件仅记录 `editors/` 相关目标、边界与检查约束。

## 1. 作用域
- 目录：`editors/`（当前重点：`editors/vscode/`）。
- 目标：提升编辑器开发体验，不与自举编译链路文档混写。

## 2. 检查约束（Editors 专用）
- 每次修改 `editors/**` 后必须运行：
  - `./editors/scripts/run_checks.sh`
- Editors 流程不要求运行：
  - `./scripts/run_checks.sh`
  - `./scripts/run_lency_checks.sh`

## 3. 当前实现状态
- 扩展入口模块化：`src/core/* + src/providers/* + src/extension.ts`。
- 模式可视化：状态栏 `Lency: LSP/Fallback`。
- LSP 路径：支持 `lency.serverPath`（含 `${workspaceFolder}`）。
- 配置热更新：修改 `lency.serverPath` 后自动重连并切换模式。

## 4. 已知边界
- `FIXME`: 格式化器对字符串/注释中的花括号仍可能误缩进。
- `TODO`: `rename/definition` 仍为单文件模型，跨文件安全语义待迁移到 LSP-first。
- `TODO`: 增加最小自动化回归（completion/definition/rename/signature/fallback diagnostics）。
