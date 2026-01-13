# Sprint 13: 自举准备 (Bootstrapping Preparation)

> **目标**: 完善标准库和语言特性，使其具备编写 Lency 编译器（自举）的能力。
> **核心关注**: **模块系统 (Import)**、集合库 (HashMap)、泛型 Traits (Eq, Hash)。

## 1. 核心需求分析 (Why?)

要用 Lency 编写 Lency 编译器，我们需要：
1.  **模块化**: 编译器代码量大，必须支持 `import` 分文件编写。
2.  **符号表**: 需要 `HashMap<String, Symbol>` -> 需要 `HashMap` -> 需要 `Hash` + `Eq` traits.
3.  **AST 处理**: 需要 `Option<T>` (处理可选子节点) 和 `Result<T, E>` (错误处理).
4.  **代码生成**: 需要字符串拼接、格式化 (`int_to_string`), 数组操作 (`Vec` 方法).

## 2. 详细规划 (What?)

### 2.1 模块系统 (Import) **[最高优先级]**
没有 import，标准库无法拆分，编译器无法编写。
-   **语法**: `import std.io`, `import std.collections.HashMap`
-   **实现**:
    -   Lexer: 已有 token `Import`。
    -   AST: 添加 `Decl::Import`。
    -   Parser: 解析 `import path;`。
    -   Resolver: 递归解析导入的模块，建立模块依赖图，防止循环依赖。
    -   Codegen: 支持多文件链接。

### 2.2 泛型 Traits 系统
必须先有这两个 Trait，才能实现 HashMap。
-   **`trait Eq<T>`**: 用于键的比较。
-   **`trait Hash`**: 用于计算哈希值 (建议 v1 实现 Java 风格 `int hashCode()`).

### 2.3 集合库: HashMap
-   **结构定义**: 链地址法实现的 `HashMap<K, V>`.
-   **方法**: `insert`, `get`, `contains_key`.

### 2.4 标准库重构 (Restructuring)
随着标准库变大，需要更好的目录结构：
```text
lib/std/
├── core/
│   ├── mod.lcy      (内置类型扩展)
│   ├── result.lcy
│   └── option.lcy
├── collections/
│   ├── mod.lcy
│   ├── vec.lcy
│   └── hash_map.lcy
├── io/
│   └── mod.lcy
└── string/
    └── mod.lcy
```

### 2.5 实用工具扩展
-   **Option/Result**: `unwrap()`, `expect()`, `is_some()`.
-   **String**: `int.to_string()`.

## 3. 任务清单 (How?)

### Phase 1: 模块系统 (The Foundation)
- [ ] AST 添加 `Decl::Import` { path: Vec<String> }.
- [ ] Parser 支持 `import std.io`.
- [ ] Resolver 实现模块查找与加载 (基于文件路径).
- [ ] 重构标准库目录结构，验证 import 功能。

### Phase 2: Traits & Hash
- [ ] 定义 `trait Eq` 和 `trait Hash`.
- [ ] 为内置类型 (int, string, bool) 实现这些 Trait.
- [ ] 验证泛型约束 `K: Hash + Eq`.

### Phase 3: HashMap
- [ ] 实现 `HashMap<K, V>` 结构体.
- [ ] 实现 `insert`, `get` 等核心方法.
- [ ] 编写测试验证哈希冲突处理.

### Phase 4: 工具链补全
- [ ] 实现 `int_to_string` (FFI).
- [ ] 完善 `Option` 和 `Result` 的 helper methods.

## 4. 风险与挑战
-   **模块循环依赖**: 需要在 Resolver 阶段检测并报错。
-   **泛型约束**: 检查编译器是否支持 `K: Hash + Eq` 这种多重约束。
