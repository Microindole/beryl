---
name: runtime
description: C 运行时与 FFI 模块 (lency_runtime) 技术手册。
---

# Lency Runtime - 运行时

## 1. 架构
- **Rust 层 (`lency_runtime`)**: 提供给编译器链接的内建函数声明。
- **C 层 (`liblency`)**: (位于运行时内部或其他目录) C 写的性能敏感代码。

## 2. 核心功能
- **String 处理**: `char_to_string`, `concat`, `trim` 等 C 层实现。
- **Collections**: `Vec`, `HashMap` 的堆管理实现。
- **I/O**: 文件读写包装。

## 3. FFI 准则
- **ABI 一致性**: 必须暴露 `extern "C"` 接口。
- **内存所有权**: 明确谁分配、谁释放。当前倾向于“运行时分配，手动释放”或引用计数。
- **Panic 处理**: 运行时发生不可恢复错误应调用 `lency_panic`。

---
[运行时入口](../../../../crates/lency_runtime/src/lib.rs) | [String Intrinsics](../../../../crates/lency_runtime/src/string.rs)
