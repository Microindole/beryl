use super::super::helpers::{ident_parser, type_parser};
use super::intrinsics;
use super::literal;
use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

use super::ParserError;

pub fn parser(
    expr: impl Parser<Token, Expr, Error = ParserError> + Clone,
) -> impl Parser<Token, Expr, Error = ParserError> + Clone {
    // 字面量
    let val = literal::literal_parser();
    // 基本原子表达式
    let ident = ident_parser().map_with_span(|name, span| Expr {
        kind: ExprKind::Variable(name),
        span,
    });

    let paren = expr
        .clone()
        .delimited_by(just(Token::LParen), just(Token::RParen));

    let match_expr = just(Token::Match)
        .ignore_then(expr.clone())
        .then(
            just(Token::Case)
                .ignore_then(crate::parser::pattern::pattern_parser())
                .then_ignore(just(Token::Arrow))
                .then(expr.clone())
                .map_with_span(|(pattern, body), span| MatchCase {
                    pattern,
                    body: Box::new(body),
                    span,
                })
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map_with_span(|(value, cases), span| Expr {
            kind: ExprKind::Match {
                value: Box::new(value),
                cases,
                default: None, // Default is handled via Wildcard pattern now
            },
            span,
        });

    // 内置函数
    let intrinsic_expr = intrinsics::intrinsic_parsers(expr.clone());

    // Array literal: [1, 2, 3]
    let array_literal = expr
        .clone()
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .delimited_by(just(Token::LBracket), just(Token::RBracket))
        .map_with_span(|elements, span| Expr {
            kind: ExprKind::Array(elements),
            span,
        });

    // Vec 字面量: vec![1, 2, 3]
    let vec_literal = just(Token::Vec)
        .ignore_then(just(Token::Bang))
        .ignore_then(
            expr.clone()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .delimited_by(just(Token::LBracket), just(Token::RBracket)),
        )
        .map_with_span(|elements, span| Expr {
            kind: ExprKind::VecLiteral(elements),
            span,
        });

    // Struct literal: Point { x: 10, y: 20 } or Box<int> { value: 10 }
    let struct_literal = type_parser()
        .then(
            ident_parser()
                .then_ignore(just(Token::Colon))
                .then(expr.clone())
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map_with_span(|(type_, fields), span| Expr {
            kind: ExprKind::StructLiteral { type_, fields },
            span,
        });

    // Ok 构造器: Ok(value)
    let ok_expr = just(Token::Ok)
        .ignore_then(
            expr.clone()
                .delimited_by(just(Token::LParen), just(Token::RParen)),
        )
        .map_with_span(|inner, span| Expr {
            kind: ExprKind::Ok(Box::new(inner)),
            span,
        });

    // Err 构造器: Err(message)
    let err_expr = just(Token::Err)
        .ignore_then(
            expr.clone()
                .delimited_by(just(Token::LParen), just(Token::RParen)),
        )
        .map_with_span(|inner, span| Expr {
            kind: ExprKind::Err(Box::new(inner)),
            span,
        });

    // 闭包: |int a, int b| => a + b
    // 参数: Type Ident
    let closure_param = type_parser()
        .then(ident_parser())
        .map(|(ty, name)| Param { name, ty });

    let closure_expr = just(Token::Pipe)
        .ignore_then(
            closure_param
                .separated_by(just(Token::Comma))
                .allow_trailing(),
        )
        .then_ignore(just(Token::Pipe))
        .then_ignore(just(Token::Arrow))
        .then(expr.clone())
        .map_with_span(|(params, body), span| Expr {
            kind: ExprKind::Closure {
                params,
                body: Box::new(body),
            },
            span,
        });

    // Unit literal: ()
    let unit = just(Token::LParen)
        .ignore_then(just(Token::RParen))
        .map_with_span(|_, span| Expr {
            kind: ExprKind::Unit,
            span,
        });

    // let atom = val.or(call).or(ident).or(paren);
    // Integrate match_expr. Should be high precedence.
    closure_expr
        .or(match_expr)
        .or(intrinsic_expr)
        .or(vec_literal)
        .or(array_literal)
        .or(ok_expr)
        .or(err_expr)
        .or(struct_literal)
        .or(unit) // Check unit () before paren (expr)
        .or(val)
        .or(ident)
        .or(paren)
}
