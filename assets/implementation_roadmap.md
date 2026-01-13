# Lency 语言实现路线图 v3.1

> **当前状态**: Sprint 12 完成 (标准库与 I/O)
> **设计哲学**: 简洁 (Concise) · 规范 (Standard) · 清晰 (Clear)
> **最后更新**: 2026-01-13

---

## 🏛️ 已完成里程碑 (Completed)

### Sprint 1-4: 语言基石
- ✅ **基础架构**: Lexer, Parser, AST, Codegen (LLVM)
- ✅ **核心类型**: `int`, `float`, `bool`, `string`
- ✅ **控制流**: `if`, `while`, `for` (Classic & For-in)
- ✅ **集合**: 数组 (`[T; N]`) 与 动态数组 (`vec![...]`)

### Sprint 5: 结构体与方法
- ✅ **结构体**: C 风格结构体定义与初始化
- ✅ **方法**: `impl` 块，支持方法定义
- ✅ **隐式 This**: 方法内字段的简写访问

### Sprint 6: 空安全系统
- ✅ **显式空类型**: `T?` 表示可空，`T` 默认非空
- ✅ **安全操作符**: `?.` (安全调用) 与 `??` (Elvis)
- ✅ **流敏感分析**: `if x != null` 智能转换

### Sprint 7: 泛型
- ✅ **泛型结构体**: `struct Box<T> { ... }`
- ✅ **泛型函数**: `T identity<T>(T x)`
- ✅ **单态化**: 编译时生成具体类型代码

### Sprint 8: 特性 (Traits)
- ✅ **Trait 定义**: 接口定义与方法签名
- ✅ **Trait 实现**: `impl Trait for Type`
- ✅ **泛型约束**: `void foo<T: Trait>(T x)`
- ✅ **方法注册**: trait/impl 方法正确注册到 StructSymbol

### Sprint 9: 错误处理
- ✅ **Result 类型**: `int!` 语法糖
- ✅ **构造器**: `Ok(val)` 与 `Err(err)`
- ✅ **Try 操作符**: `expr?` 自动解包或提前返回

### Sprint 10: ADTs 与 模式匹配
- ✅ **枚举定义**: `enum Color { Red, Green, Blue }`
- ✅ **Tuple 变体**: `enum Option { Some(T), None }`
- ✅ **模式匹配**: `match` 表达式支持枚举解构
- ✅ **穷尽性检查**: 报告缺失的枚举变体

### Sprint 11: 闭包与函数类型
- ✅ **函数类型**: `int(int, int)` (C 风格)
- ✅ **闭包语法**: `|int x| => x * 2`
- ✅ **闭包提升**: 编译为匿名顶层函数
- ✅ **间接调用**: 函数指针变量调用

### Sprint 12: 标准库与 I/O ✅
- ✅ **文件 I/O**: `read_file(path)` → `string!`, `write_file(path, content)` → `void!`
- ✅ **字符串操作**: `len`, `trim`, `split`, `join`, `substr`
- ✅ **标准库文件**: `lib/std/{core,io,collections,string}.lcy`
- ✅ **运行时 FFI**: `lency_runtime/src/{file,string}.rs`

---

## 📅 开发时间表

| 冲刺 | 核心任务 | 状态 |
|------|---------|------|
| Sprint 1-6 | 基础/OOP/空安全 | ✅ 完成 |
| Sprint 7 | 泛型 | ✅ 完成 |
| Sprint 8 | Trait 系统 | ✅ 完成 |
| Sprint 9 | 错误处理 | ✅ 完成 |
| Sprint 10 | ADTs & 模式匹配 | ✅ 完成 |
| Sprint 11 | 闭包与函数类型 | ✅ 完成 |
| Sprint 12 | 标准库 & I/O | ✅ 完成 |
| **Sprint 13** | **自举准备** | 📋 下一步 |
| Sprint 14 | 编译器前端自举 | 📋 规划中 |

---

## 🎯 Sprint 13: 自举准备

### 核心目标
让 Lency 能够用自己写编译器的核心组件

### 13.1 Hash 支持 (自举必需)
- [ ] `Hash` trait 定义
- [ ] 基础类型 Hash 实现 (`int`, `string`)
- [ ] `HashMap<K, V>` 运行时实现
- [ ] HashMap 内置函数

### 13.2 标准库扩充
- [ ] `Option<T>` 枚举 (泛型 enum)
- [ ] `Vec<T>` 方法 (`len`, `get`, `push`, `pop`, `last`)
- [ ] 类型转换函数 (`int_to_string`, `parse_int`)
- [ ] Result 方法 (`is_ok`, `is_err`, `unwrap`)

### 13.3 语言特性补全
- [ ] 泛型枚举完整支持
- [ ] 泛型相等比较 (`Eq` trait)
- [ ] 模块导入系统 (`import`)

---

## 📊 项目统计

| 指标 | 数值 |
|------|------|
| 代码行数 | ~13,300 |
| Rust 文件 | 109 |
| 测试用例 | 64 |
| 标准库文件 | 4 |
| 集成测试目录 | 12 |

---

## 🔧 待解决问题

### TODO 标记 (15 个)
主要分布在：
- 标准库 (11 个) - 需要更多语言特性
- 类型推导 (3 个) - LUB 算法等
- 测试 (1 个) - unwrap 实现

### 代码质量
- 9 个文件超过 300 行 (但都在可接受范围)
- 无超过 500 行的文件
