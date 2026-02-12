# TODO/FIXME 清理计划

## 当前状态
13 个 TODO + 3 个 FIXME = 16 个标记

## 分类

### A. 可直接清理 (5个)

| # | 文件 | 内容 | 操作 |
|---|------|------|------|
| 1 | [basic.lcy](file:///home/indolyn/beryl/tests/integration/error/basic.lcy#L28) | `TODO: Assert results (需实现 unwrap)` | unwrap 已实现，补上断言并删除 TODO |
| 2 | [conversion_test.lcy](file:///home/indolyn/beryl/tests/integration/stdlib/conversion_test.lcy#L34) | `TODO: print int directly` | print 已支持 int，补全测试 |
| 3 | [vec.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/expr/vec.rs#L3) | `TODO: Vec 方法支持尚未完全实现` | push/pop/len/get/set 全部已实现，删除过时注释和 `#![allow(dead_code)]` |
| 4 | [lib.rs](file:///home/indolyn/beryl/crates/lency_monomorph/src/lib.rs#L90) | `_ => "unknown"` TODO | 改为 `unreachable!()` 或转为 `panic!` 提供更好的报错 |
| 5 | [control.rs](file:///home/indolyn/beryl/crates/lency_sema/src/type_infer/control.rs#L229) | `TODO: move to a shared utility` | 方法只在此文件使用，删除 TODO，标注为内部辅助函数 |

### B. 保留 (留意但不删除) (5个)

| # | 文件 | 内容 | 理由 |
|---|------|------|------|
| 6 | [operators.rs](file:///home/indolyn/beryl/crates/lency_sema/src/type_infer/operators.rs#L23) | LUB 类型推导 | 需要设计工作，非简单修复 |
| 7 | [functions.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/module/functions.rs#L170) | global init handling | 全局变量初始化是复杂特性 |
| 8 | [array.rs](file:///home/indolyn/beryl/crates/lency_codegen/src/expr/array.rs#L126) | String 索引边界检查 | 需要生成运行时 strlen 调用，待 Sprint 16+ |
| 9 | [collections.lcy](file:///home/indolyn/beryl/lib/std/collections.lcy#L137) | HashMapIntInt 包装不可用 | codegen struct 返回值问题，非本次范围 |
| 10 | [core.lcy](file:///home/indolyn/beryl/lib/std/core.lcy#L236) | parse_int_strict | 需要 Result FFI 支持，非本次范围 |

### C. 文档引用 (不可操作) (6个)

`context.md`、`management/SKILL.md`(3处)、`status.md`(2处) 中出现的 TODO/FIXME 文字是描述性内容，不是代码标记。

> [!NOTE]
> 文档引用中的 TODO/FIXME 字符串会被 `run_checks.sh` 计入统计，但它们不是实际技术债。

## 预期效果
- TODO: 13 -> 8 (减少 5 个)
- FIXME: 3 -> 3 (全为文档引用)
