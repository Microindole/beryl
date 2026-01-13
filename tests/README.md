# Lency 测试结构说明

## 目录组织

```
tests/
├── integration/          # 集成测试（Lency 代码）
│   ├── arrays/          # 数组相关测试
│   ├── structs/         # 结构体相关测试
│   ├── methods/         # 方法相关测试
│   └── ...              # 其他特性测试
└── unit/                # 单元测试（Rust 代码）
    └── (future)         # 未来的 Rust 单元测试
```

##用法

### 运行集成测试
```bash
# 运行单个测试
cargo run --bin lencyc -- run tests/integration/structs/struct_test.lency

# 运行所有数组测试
for f in tests/integration/arrays/*.lcy; do
    cargo run --bin lencyc -- run "$f"
done
```

### 添加新测试
1. 在对应的子目录创建 `.lcy` 或 `.lency` 文件
2. 使用 `lencyc run` 或 `lencyc check` 验证

## 迁移说明
- 所有 `examples/*.lcy` 已移至 `tests/integration/`
- 所有 `tests/*.lency` 已移至 `tests/integration/structs/`
- 原 `examples/` 文件夹已删除
