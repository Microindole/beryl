# Sprint 17 Tasks: Bootstrap - Parser

- [ ] **AST 定义** (`lencyc/syntax/ast.lcy`)
    - [ ] `Node` interface/trait (if applicable) or Base Enum
    - [ ] `Expr` enum (Binary, Unary, Literal, etc.)
    - [ ] `Stmt` enum (If, While, VarDecl, etc.)
    - [ ] `Type` representation

- [ ] **Parser 基础** (`lencyc/syntax/parser.lcy`)
    - [ ] `struct Parser`
    - [ ] `match`, `consume`, `check` helper methods
    - [ ] Error synchronization logic

- [ ] **Expression Parsing**
    - [ ] Precedence table/logic
    - [ ] `parse_precedence` (Pratt) or recursive structure
    - [ ] Leaf nodes (literals, identifiers)
    - [ ] Infix/Prefix operators

- [ ] **Statement Parsing**
    - [ ] `parse_decl` (var, func)
    - [ ] `parse_stmt` (if, while, block, return, expr_stmt)

- [ ] **验证 & 驱动**
    - [ ] AST Pretty Printer (`ast_to_string`)
    - [ ] 更新 `lencyc/driver/main.lcy`
    - [ ] 运行测试

---

# Completed (Sprint 16)
- [x] Token 定义
- [x] Keywords 映射
- [x] Lexer 核心逻辑
- [x] Driver 验证
