use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

// 定义解析器错误类型
pub type ParserError = Simple<Token>;

/// 主入口：解析整个程序
pub fn program_parser() -> impl Parser<Token, Program, Error = ParserError> {
    decl_parser()
        .repeated()
        .map(|decls| Program { decls })
        .then_ignore(end())
}

/// 解析声明 (Function, Class)
fn decl_parser() -> impl Parser<Token, Decl, Error = ParserError> {
    recursive(|decl| {
        // --- 1. 函数声明 ---
        // int add(int a, int b) { ... }
        let func = type_parser()
            .then(ident_parser())
            .then(
                type_parser()
                    .then(ident_parser())
                    .map(|(ty, name)| Param { name, ty })
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::LParen), just(Token::RParen))
            )
            // 修复: body 需要 Vec<Stmt>，所以要 repeated()
            .then(stmt_parser().repeated().delimited_by(just(Token::LBrace), just(Token::RBrace)))
            .map_with_span(|(((return_type, name), params), body), span| {
                Decl::Function {
                    span,
                    name,
                    params,
                    return_type,
                    body, // 这里 body 是 Vec<Stmt>
                }
            });

        // --- 2. 类声明 ---
        // class User { ... }
        let class_decl = just(Token::Class)
            .ignore_then(ident_parser())
            .then(
                ident_parser()
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::Lt), just(Token::Gt))
                    .or_not()
                    .map(|g| g.unwrap_or_default())
            )
            .then(
                field_parser()
                    .repeated()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace))
            )
            .map_with_span(|((name, generics), fields), span| {
                Decl::Class {
                    span,
                    name,
                    generics,
                    fields,
                    methods: vec![],
                }
            });

        class_decl.or(func)
    })
}

/// 解析语句
/// 返回 opaque type 时加上 Clone，解决 recursive 中 .clone() 的问题
fn stmt_parser() -> impl Parser<Token, Stmt, Error = ParserError> + Clone {
    recursive(|stmt| {
        // 提取 Block 逻辑 (返回 Vec<Stmt>)，供 Stmt::Block 和 If 使用
        let raw_block = stmt.clone()
            .repeated()
            .delimited_by(just(Token::LBrace), just(Token::RBrace));

        // 变量声明: var x: int = 1;
        let var_decl = just(Token::Var)
            .ignore_then(ident_parser())
            .then(just(Token::Colon).ignore_then(type_parser()).or_not())
            .then_ignore(just(Token::Eq))
            .then(expr_parser())
            .then_ignore(just(Token::Semicolon).or_not())
            // 修复字段名: ty, value
            .map_with_span(|((name, ty), value), span| Stmt::VarDecl {
                span,
                name,
                ty,      // 原 type_annotation
                value,   // 原 initializer
            });

        // 块语句: { ... }
        let block_stmt = raw_block.clone().map(Stmt::Block);

        // Return
        let ret = just(Token::Return)
            .ignore_then(expr_parser().or_not())
            .then_ignore(just(Token::Semicolon).or_not())
            // 修复闭包参数: |value, span|
            .map_with_span(|value, span| Stmt::Return { span, value });

        // If
        let if_stmt = just(Token::If)
            .ignore_then(expr_parser()) 
            .then(raw_block.clone()) // then block 是 Vec<Stmt>
            .then(just(Token::Else).ignore_then(raw_block.clone()).or_not()) // else block
            // 修复字段名: then_block, else_block
            .map_with_span(|((condition, then_block), else_block), span| {
                Stmt::If { 
                    span, 
                    condition, 
                    then_block, // Vec<Stmt>
                    else_block  // Option<Vec<Stmt>>
                }
            });
        
        // While
        let while_stmt = just(Token::While)
            .ignore_then(expr_parser())
            .then(raw_block.clone())
            .map_with_span(|(condition, body), span| Stmt::While {
                span,
                condition,
                body
            });

        // 表达式语句
        let expr_stmt = expr_parser()
            .then_ignore(just(Token::Semicolon).or_not())
            .map(Stmt::Expression);

        var_decl
            .or(block_stmt)
            .or(ret)
            .or(if_stmt)
            .or(while_stmt)
            .or(expr_stmt)
    })
}

/// 解析表达式
fn expr_parser() -> impl Parser<Token, Expr, Error = ParserError> + Clone {
    recursive(|expr| {
        let val = select! {
            Token::Int(x) => Literal::Int(x),
            Token::Float(s) => Literal::Float(s.parse().unwrap_or(0.0)),
            Token::String(s) => Literal::String(s),
            Token::True => Literal::Bool(true),
            Token::False => Literal::Bool(false),
            Token::Null => Literal::Null,
        }.map_with_span(|lit, span| Expr { kind: ExprKind::Literal(lit), span });

        let atom = val
            .or(ident_parser().map_with_span(|name, span| Expr { kind: ExprKind::Variable(name), span }))
            .or(expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)));

        let product = atom.clone().then(
            just(Token::Star).to(BinaryOp::Mul)
                .or(just(Token::Slash).to(BinaryOp::Div))
                .or(just(Token::Percent).to(BinaryOp::Mod))
                .then(atom)
                .repeated()
        ).foldl(|lhs, (op, rhs)| {
            let span = lhs.span.start..rhs.span.end;
            Expr { kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)), span }
        });

        let sum = product.clone().then(
            just(Token::Plus).to(BinaryOp::Add)
                .or(just(Token::Minus).to(BinaryOp::Sub))
                .then(product)
                .repeated()
        ).foldl(|lhs, (op, rhs)| {
            let span = lhs.span.start..rhs.span.end;
            Expr { kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)), span }
        });

        sum
    })
}

/// 辅助：解析标识符
fn ident_parser() -> impl Parser<Token, String, Error = ParserError> + Clone {
    select! { Token::Ident(ident) => ident }
}

/// 辅助：解析类型
/// 移除无用的 recursive，除非我们要处理 Generic<T>
fn type_parser() -> impl Parser<Token, Type, Error = ParserError> + Clone {
    let basic = select! {
        Token::TypeInt => Type::Int,
        Token::TypeFloat => Type::Float,
        Token::TypeString => Type::String,
        Token::TypeBool => Type::Bool,
        Token::TypeVoid => Type::Void,
        Token::Ident(name) => Type::Class(name),
    };

    basic.then(just(Token::Question).or_not())
        .map(|(t, q)| if q.is_some() { Type::Nullable(Box::new(t)) } else { t })
}

/// 辅助：解析字段
fn field_parser() -> impl Parser<Token, Field, Error = ParserError> + Clone {
    type_parser()
        .then(ident_parser())
        .then_ignore(just(Token::Semicolon).or_not())
        .map(|(ty, name)| Field { name, ty })
}