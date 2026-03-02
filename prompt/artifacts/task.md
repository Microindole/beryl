# Sprint 17 Tasks: Bootstrap - Parser

- [ ] **AST 定义** (`lencyc/syntax/ast.lcy`)
    - [x] `Expr` 基础节点 (Binary, Unary, Literal, Variable, Assign, Logical)
    - [x] `Stmt` 基础节点 (If, While, VarDecl, Block, Return, Expr)
    - [ ] `Type` representation

- [x] **Parser 基础** (`lencyc/syntax/parser.lcy`)
    - [x] `struct Parser`
    - [x] `match`, `consume`, `check` helper methods
    - [ ] Error synchronization logic

- [ ] **Expression Parsing**
    - [x] 递归优先级链 (`assignment -> or -> and -> equality -> comparison -> term -> factor -> unary -> primary`)
    - [x] Leaf nodes (number, bool, identifier, grouping)
    - [x] Infix/Prefix operators
    - [ ] 字符串/浮点等字面量扩展

- [ ] **Statement Parsing**
    - [x] `parse_decl` (var)
    - [x] `parse_stmt` (if, while, block, return, expr_stmt)
    - [ ] `func/struct/impl` 声明解析

- [ ] **验证 & 驱动**
    - [x] AST Pretty Printer (`expr_to_string` / `stmt_to_string`)
    - [x] 更新 `lencyc/driver/test_entry.lcy` (覆盖 `return` 解析路径)
    - [x] 运行 `./scripts/run_lency_checks.sh`

---

# Completed (Sprint 16)
- [x] Token 定义
- [x] Keywords 映射
- [x] Lexer 核心逻辑
- [x] Driver 验证
