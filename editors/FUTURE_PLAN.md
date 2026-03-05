# Lency 编辑器演进路线

本路线遵循 Lency 设计哲学：简洁、规范、清晰、安全优先。编辑器侧不做“黑魔法”推断，能力以可解释、可验证为准。

## Phase 1（当前）

目标：稳定的 VS Code 基础体验。

- 已完成：
  - LSP 启动降级（有 `lency_ls` 则启用，无则本地 fallback）
  - 单文件定义跳转/重命名/符号
  - 内建函数补全、hover、签名提示
  - 基础格式化
  - fallback 括号匹配诊断
- `FIXME`: 格式化器在字符串/注释含花括号时会误判缩进。
- `TODO`: fallback 诊断暂未覆盖语法级错误（仅括号匹配）。

## Phase 2（近期）

目标：把“单文件正则能力”替换为“可复用语法层”。

- 引入轻量语法树或可复用 parser 接口，替代扩展内正则匹配。
- 补齐语法诊断：未闭合字符串、非法 token、关键字误用等。
- 格式化器升级为 token-aware，消除已知 `FIXME`。
- `TODO`: 为扩展增加最小自动化测试（completion/rename/definition 的回归样例）。

## Phase 3（中期）

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
