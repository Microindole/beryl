# Sprint 13: 自举准备 (Bootstrapping Preparation)

> **目标**: 完善标准库和语言特性，使其具备编写 Beryl 编译器（自举）的能力。
> **核心关注**: 集合库 (HashMap)、泛型 Traits (Eq, Hash) 和 常用工具函数。

## 1. 核心需求分析 (Why?)

要用 Beryl 编写 Beryl 编译器，我们需要：
1.  **符号表**: 需要 `HashMap<String, Symbol>` -> 需要 `HashMap` -> 需要 `Hash` + `Eq` traits.
2.  **AST 处理**: 需要 `Option<T>` (处理可选子节点) 和 `Result<T, E>` (错误处理).
3.  **代码生成**: 需要字符串拼接、格式化 (`int_to_string`), 数组操作 (`Vec` 方法).

## 2. 详细规划 (What?)

### 2.1 泛型 Traits 系统 (基础)
必须先有这两个 Trait，才能实现 HashMap。

-   **`trait Eq<T>` (或 `Comparable`)**
    -   用于键的比较。
    -   需要支持泛型约束: `struct HashMap<K: Eq + Hash, V>` (Beryl 目前支持多重约束吗？如果不支持，可能需要合并为一个 `Key` trait).
    -   *任务*: 实现 `impl Eq for int`, `impl Eq for string`.

-   **`trait Hash`**
    -   用于计算哈希值。
    -   设计方案: Java 风格 `int hashCode()` vs Rust 风格 `void hash(Hasher state)`.
    -   *建议*: 采用 Java 风格 `int hashCode()` 作为 v1 实现，简单直接。结合 `hash_combine` 内置函数处理组合类型。
    -   *任务*: 实现 `impl Hash for int`, `impl Hash for string`.

### 2.2 集合库: HashMap
-   **结构定义**:
    ```beryl
    struct HashMap<K, V> {
        Vec<Vec<Pair<K, V>>> buckets  // 链地址法解决冲突
        int size
    }
    ```
-   **方法**:
    -   `new()`
    -   `insert(key, value)`
    -   `get(key) -> V?` (需要 Option)
    -   `contains_key(key) -> bool`

### 2.3 标准库扩充 (实用工具)
-   **Option 枚举**:
    -   完善 `enum Option<T> { Some(T), None }` 的支持。
    -   实现辅助方法: `is_some()`, `unwrap()`.
-   **Result 增强**:
    -   实现 `unwrap()`, `expect(msg)`.
-   **字符串转换**:
    -   `int.to_string()` (或 `str(int)`).
    -   这对于生成 IR 代码至关重要 (如生成 `%1`, `%2`).

## 3. 任务清单 (How?)

### Phase 1: Traits 基础
- [ ] 定义 `trait Eq` 并为内置类型实现 (int, string, bool).
- [ ] 定义 `trait Hash` 并为内置类型实现 (int, string).
- [ ] **验证**: 编写测试，确保可以将 `string` 赋值给 `trait Eq` 类型的变量，并调用方法。

### Phase 2: HashMap 原型
- [ ] 实现 `HashMap<K, V>` 结构体 (使用简单的线性探测或链地址).
- [ ] 处理泛型约束检查 (确保 K 实现了 Hash + Eq).
- [ ] **验证**: 单元测试 `map.insert("x", 1); map.get("x")`.

### Phase 3: 工具链补全
- [ ] 实现 `int_to_string` 和 `float_to_string` (可能需要 FFI).
- [ ] 完善 `Option` 和 `Result` 的 helper methods.
- [ ] 给 `Vec` 添加 `last()`, `is_empty()` 等方法 (目前部分只能通过 FFI 间接调用).

### Phase 4: 综合测试
- [ ] 编写一个简单的 "Symbol Table" 模拟程序，测试 HashMap 和 String 的综合使用。

## 4. 风险与挑战
-   **泛型约束**: 检查编译器是否支持 `K: Hash + Eq` 这种多重约束。如果不支持，需要临时定义 `trait Key: Hash + Eq` 或者重构编译器。
-   **哈希算法**: 字符串哈希需要高效，可能需要在 Runtime (Rust) 中实现 SIPHash/FNV，然后暴露给 Beryl。
