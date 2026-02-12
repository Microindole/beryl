# Sprint 15 收尾工作

## 文档清理
- [x] 清理 `status.md` 中的 emoji
- [x] 清理 `plan_15.md` 中的 emoji

## String 格式化实现 (format 函数)
- [x] 调研现有代码基础
- [x] 制定实现计划 (已批准)
- [x] Lexer: 新增 `Token::Format`
- [x] AST: 新增 `ExprKind::Format`
- [x] Parser: 添加 format 解析器
- [x] Sema: resolver + type_infer
- [x] Monomorph: specializer + collector
- [x] Runtime: `lency_string_format` FFI
- [x] Codegen: `gen_format` + 路由
- [x] 集成测试: `string_format.lcy`
- [x] 运行 `./scripts/run_checks.sh` -- 全部通过
- [x] 更新 `context.md` 和 `status.md`
