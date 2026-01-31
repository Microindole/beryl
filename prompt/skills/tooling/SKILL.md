---
name: tooling
description: 负责 IDE 演进 (Regex->LSP)、包管理器设计及工具链生态。
---

# 工具链与生态 (Tooling & Ecosystem) - 高级版

## 1. IDE 演进流水线 (Evolution Path)
遵循从“词法高亮”到“语义理解”的递进路线：

| 阶段 | 核心技术 | 实现目标 | 状态 |
|------|---------|---------|------|
| **1. 词法级** | TextMate (Regex) | 基本着色、缩进、片段 (Snippet) | ✅ 已上线 |
| **2. 语义级** | **LSP Server** | 悬停提示 (Hover)、跳转定义、自动补全 | 🏗️ 开发中 |
| **3. 集成级** | VS Code Client | 任务运行 (Tasks)、调试器 (DAP)、问题面板 | 🔶 部分实现 |

## 2. 包管理器 (Package Manager - `lcy-pkg`)
虽然尚未完全启动工作，但所有设计必须遵循：
- **清单文件**: `Lency.toml` (类似 Cargo.toml)。
- **核心逻辑**:
    - **依赖解析**: 基于版本的语义化解析。
    - **构建集成**: 与 `lencyc` 编译器后端深度绑定。
    - **仓库协议**: 预留中心化 Registry 与 Git 源支持。

## 3. 开发者核心菜谱 (Ecosystem Recipes)
| ID | 任务 | 操作路径 |
|----|------|---------|
| **T1** | **词法高亮更新** | `editors/vscode/syntaxes/lency.tmLanguage.json` |
| **T2** | **LSP 节点扩展** | `lencer-ls` (待建) -> `lency_sema` 接口。 |
| **T3** | **诊断信息优化** | 修改 `lency_diagnostics` 格式 -> 更新插件 `problemMatchers`。 |
| **T4** | **包管理设计** | 制定 `Lency.toml` 规范 -> 参考 `assets/Lency.txt`。 |

## 4. 避坑指南 (Pitfalls)
- **正则冲突**: TextMate 正则过于复杂会导致 VS Code 渲染性能下降。
- **LSP 同步**: LSP 状态必须与 `lency_sema` 的 `AnalysisResult` 保持强一致，否则补全会错位。
- **ABI 溢出**: 跨包依赖时注意 `lency_monomorph` 产生的单态化符号合并冲突。

---
[VS Code 配置](../../../editors/vscode/package.json) | [工具链蓝图](../../../editors/FUTURE_PLAN.md)
