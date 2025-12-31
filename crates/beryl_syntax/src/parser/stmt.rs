//! Statement Parser
//!
//! 语句解析：变量声明、赋值、return、if、while、block等

use super::expr::expr_parser;
use super::helpers::{ident_parser, type_parser};
use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

pub type ParserError = Simple<Token>;

/// 解析语句 (公共接口)
pub fn stmt_parser() -> impl Parser<Token, Stmt, Error = ParserError> + Clone {
    recursive(|stmt| {
        // Block 逻辑 (返回 Vec<Stmt>)
        let raw_block = stmt
            .clone()
            .repeated()
            .delimited_by(just(Token::LBrace), just(Token::RBrace));

        // 变量声明: var x: int = 1;
        let var_decl = just(Token::Var)
            .ignore_then(ident_parser())
            .then(just(Token::Colon).ignore_then(type_parser()).or_not())
            .then_ignore(just(Token::Eq))
            .then(expr_parser())
            .then_ignore(just(Token::Semicolon).or_not())
            .map_with_span(|((name, ty), value), span| Stmt::VarDecl {
                span,
                name,
                ty,
                value,
            });

        // 赋值语句: x = 10;
        let assignment = ident_parser()
            .then_ignore(just(Token::Eq))
            .then(expr_parser())
            .then_ignore(just(Token::Semicolon).or_not())
            .map_with_span(|(name, value), span| {
                let target_span = span.clone();
                Stmt::Assignment {
                    span,
                    target: Expr {
                        kind: ExprKind::Variable(name),
                        span: target_span,
                    },
                    value,
                }
            });

        // 块语句: { ... }
        let block_stmt = raw_block.clone().map(Stmt::Block);

        // Return
        let ret = just(Token::Return)
            .ignore_then(expr_parser().or_not())
            .then_ignore(just(Token::Semicolon).or_not())
            .map_with_span(|value, span| Stmt::Return { span, value });

        // If
        let if_stmt = just(Token::If)
            .ignore_then(expr_parser())
            .then(raw_block.clone())
            .then(just(Token::Else).ignore_then(raw_block.clone()).or_not())
            .map_with_span(|((condition, then_block), else_block), span| Stmt::If {
                span,
                condition,
                then_block,
                else_block,
            });

        // While
        let while_stmt = just(Token::While)
            .ignore_then(expr_parser())
            .then(raw_block.clone())
            .map_with_span(|(condition, body), span| Stmt::While {
                span,
                condition,
                body,
            });

        // 表达式语句
        let expr_stmt = expr_parser()
            .then_ignore(just(Token::Semicolon).or_not())
            .map(Stmt::Expression);

        var_decl
            .or(assignment)
            .or(block_stmt)
            .or(ret)
            .or(if_stmt)
            .or(while_stmt)
            .or(expr_stmt)
    })
}
