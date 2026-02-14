# Sprint 17: 自举 - 语法分析器 (Parser)

**目标**: 实现一个递归下降解析器 (Recursive Descent Parser)，将 Token 流转换为抽象语法树 (AST)。

## 核心任务

### 1. AST 定义 (`lencyc/syntax/ast.lcy`)
- [ ] **Expression Nodes**: Binary, Unary, Literal, Variable, Call, Grouping
- [ ] **Statement Nodes**: ExpressionStmt, VarDecl, FunctionDecl, Block, If, While, Return
- [ ] **Type Nodes**: TypeRef (Named, Generic)

### 2. Parser 基础架构 (`lencyc/syntax/parser.lcy`)
- [ ] `struct Parser` (tokens, current, panic_mode)
- [ ] `match(token_type)`, `consume(token_type, message)`
- [ ] `synchronize()` (错误恢复)

### 3. Expression Parsing
- [ ] 实现优先级逻辑 (Equality, Comparison, Term, Factor, Unary, Call)
- [ ] `parse_expression()`

### 4. Statement Parsing
- [ ] `parse_declaration()`
- [ ] `parse_var_declaration()`
- [ ] `parse_function_declaration()`
- [ ] `parse_statement()` (If, While, Block, Return)

### 5. AST Printer (Pretty Printer)
- [ ] 实现 `ast_to_string(Node)`用于调试验证

### 6. 驱动程序更新
- [ ] 更新 `main.lcy` 调用 Parser 并输出 AST 结构

## 依赖
- Lexer (Sprint 16 Done)
- Generic Enums/Structs (Supported)
- Result/Option (Supported)

## 风险
- 递归深度过深可能导致栈溢出 (需注意测试)
- 错误恢复机制的复杂性

## 预计产出
- `lencyc/syntax/ast.lcy`
- `lencyc/syntax/parser.lcy`
- `lencyc/driver/main.lcy` (updated)
