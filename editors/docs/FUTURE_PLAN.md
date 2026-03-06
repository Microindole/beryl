# Lency 编辑器演进路线

本路线遵循 Lency 设计哲学：简洁、规范、清晰、安全优先。编辑器侧不做"黑魔法"推断，能力以可解释、可验证为准。

## Phase 1（已完成）

- LSP 启动降级（有 `lency_ls` 则启用，无则本地 fallback）
- 单文件定义跳转/重命名/符号
- 内建函数补全、hover、签名提示
- 基础格式化
- fallback 括号匹配诊断
- 格式化器字符串/注释花括号误缩进修复

## Phase 2（已完成）

目标：把"单文件正则能力"替换为"可复用语法层"。

- 格式化器升级为 token-aware（字符串/注释内花括号不再误判）
- 补齐语法诊断：未闭合字符串、全角标点误用
- fallback rename/definition 升级为跨工作区文件
- 最小自动化回归测试 19 个用例（completion/signature/definition/rename/fallback diagnostics）

## Phase 3（近期）

目标：以 Rust LLS 为主，VS Code 扩展只做客户端壳层。

- 基于 `tower-lsp` 实现统一 LLS。
- 扩展切换到 LSP-first：定义、引用、重命名、诊断均由服务器提供。
- 跨文件符号索引与引用查找。
- 背景语义检查与增量更新。
- `TODO`: 明确 LLS 与 `lencyc`/Rust 主编译器共享前端模块的边界。

## Phase 4（远期）

目标：工程化 DX（调试、重构、自动修复）。

- Code Actions（missing import、样板补全）。
- 重构操作（extract method、change signature）。
- DAP 调试链路（断点、变量、调用栈）。
- `TODO`: 完成 DAP 与 LLVM 调试信息映射方案文档。
