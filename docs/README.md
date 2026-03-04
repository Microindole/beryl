# Lency 语言文档

欢迎使用 Lency 编程语言！

## 快速开始

```lency
int main() {
    print("Hello, Lency!")
    return 0
}
```

## 文档目录

### 基础
- [变量与类型](./basics/variables.md)
- [函数](./basics/functions.md)
- [控制流](./basics/control-flow.md)

### 类型系统
- [基本类型](./types/primitives.md)
- [结构体](./types/structs.md)
- [枚举](./types/enums.md)
- [Null 安全](./types/null-safety.md)

### 标准库
- [Vec](./stdlib/vec.md)
- [字符串操作](./stdlib/string.md)
- [文件 I/O](./stdlib/file-io.md)
- [HashMap](./stdlib/hashmap.md)

---

## 语言特性 (当前完成度: ~65%)

Lency 目前正处于积极开发阶段。核心语法和语义分析基本稳定，但部分高级特性仍在完善中。

| 特性 | 状态 | 备注 |
|------|------|------|
| 静态类型 | ✅ | |
| 泛型（单态化） | ✅ | 结构体泛型稳定 |
| Null 安全 | ✅ | 智能类型推断和 Elvis 操作符 |
| 模式匹配 | ✅ | 基础 Enum 支持 |
| Trait | ✅ | |
| 泛型枚举 | ⚠️ | 目前 `Option<T>` 等泛型枚举有限制 |
| 错误处理 (Result) | ⚠️ | 语法支持已具有，Result 模式待完善 |
| 内存回收 (GC) | ⚠️ | 手动管理 + LLVM 优化阶段，准备集成 Boehm GC |

### 待实现核心功能 (TODO)

1.  **完善的错误处理 (Result 模式)**：专门针对 Error Handling 的 `Result` 类型机制待进一步打磨，替代传统的 Try-Catch。
2.  **泛型枚举的全面支持**：解决目前带数据的泛型枚举在使用上的限制。
3.  **内存管理集成**：集成统一的垃圾回收或分配机制。
4.  **标准库扩展**：如 JSON 解析库等模块的支持。
5.  **自举 (Bootstrapping)**：使用 Lency 语言重写本身的 `Lexer` 和 `Parser` 是当前的阶段核心目标。

## 自举阶段能力快照（2026-03-04）

- `lencyc` 自举 Lexer 已支持字符串字面量（双引号）扫描。
- `lencyc` 自举 Parser 已支持字符串字面量进入 `primary` 表达式路径。
- `lencyc` 自举 Lexer 已支持浮点/科学计数法扫描（如 `3.14`, `1.23e-4`）。
- `lencyc` 自举主入口已具备最小完整流程：读取、词法、语法、语义与 AST 文本产物输出。
- `lencyc` 新增最小 LIR 文本发射能力：可通过 `--emit-lir` 输出 LIR（用于后续 Rust LLVM backend 对接）。
- Rust 宿主侧已增加最小 LIR 编译通路：`lencyc build <file.lir>` 可将 LIR 编译为可执行文件（当前覆盖最小指令子集）。
- Rust `.lir` backend 已支持最小外部函数调用 lowering（`call %foo(...)` -> LLVM `declare/call`）。
- `tests/example` 已扩展 LIR 回归样例（`unary_logic`、`break_continue`）并纳入自举检查脚本。
- 新增 `scripts/lency_selfhost_build.sh`：支持 `.lcy -> self-host emit-lir -> Rust build executable` 一键构建闭环。
- 新增 `scripts/lency_selfhost_run.sh`：支持 `.lcy -> self-host build -> run` 一键运行闭环（可透传程序参数）。
