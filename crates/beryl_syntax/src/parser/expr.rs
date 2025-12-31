//! Expression Parser
//!
//! 表达式解析：字面量、变量、运算符、函数调用等

use super::helpers::ident_parser;
use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

pub type ParserError = Simple<Token>;

/// 解析表达式 (公共接口)
pub fn expr_parser() -> impl Parser<Token, Expr, Error = ParserError> + Clone {
    recursive(|expr| {
        // 字面量
        let val = select! {
            Token::Int(x) => Literal::Int(x),
            Token::Float(s) => Literal::Float(s.parse().unwrap_or(0.0)),
            Token::String(s) => Literal::String(s),
            Token::True => Literal::Bool(true),
            Token::False => Literal::Bool(false),
            Token::Null => Literal::Null,
        }
        .map_with_span(|lit, span| Expr {
            kind: ExprKind::Literal(lit),
            span,
        });

        // 基本原子表达式
        let ident = ident_parser().map_with_span(|name, span| Expr {
            kind: ExprKind::Variable(name),
            span,
        });

        let paren = expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen));

        // 函数调用: func(arg1, arg2)
        let call = ident_parser()
            .then(
                expr.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .map_with_span(|(name, args), span| Expr {
                kind: ExprKind::Call {
                    callee: Box::new(Expr {
                        kind: ExprKind::Variable(name),
                        span: span.clone(),
                    }),
                    args,
                },
                span,
            });

        let atom = val.or(call).or(ident).or(paren);

        // 一元运算符 (!, -)
        let unary = just(Token::Bang)
            .to(UnaryOp::Not)
            .or(just(Token::Minus).to(UnaryOp::Neg))
            .repeated()
            .then(atom.clone())
            .foldr(|op, rhs| {
                let span = rhs.span.clone();
                Expr {
                    kind: ExprKind::Unary(op, Box::new(rhs)),
                    span,
                }
            })
            .or(atom);

        // 乘除模
        let product = unary
            .clone()
            .then(
                just(Token::Star)
                    .to(BinaryOp::Mul)
                    .or(just(Token::Slash).to(BinaryOp::Div))
                    .or(just(Token::Percent).to(BinaryOp::Mod))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.start..rhs.span.end;
                Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span,
                }
            });

        // 加减
        let sum = product
            .clone()
            .then(
                just(Token::Plus)
                    .to(BinaryOp::Add)
                    .or(just(Token::Minus).to(BinaryOp::Sub))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.start..rhs.span.end;
                Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span,
                }
            });

        // 比较运算符 (<, >, <=, >=, ==, !=)
        let comparison = sum
            .clone()
            .then(
                just(Token::Lt)
                    .to(BinaryOp::Lt)
                    .or(just(Token::Gt).to(BinaryOp::Gt))
                    .or(just(Token::Leq).to(BinaryOp::Leq))
                    .or(just(Token::Geq).to(BinaryOp::Geq))
                    .or(just(Token::EqEq).to(BinaryOp::Eq))
                    .or(just(Token::NotEq).to(BinaryOp::Neq))
                    .then(sum)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.start..rhs.span.end;
                Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span,
                }
            });

        // 逻辑与 (&&)
        let logical_and = comparison
            .clone()
            .then(
                just(Token::And)
                    .to(BinaryOp::And)
                    .then(comparison)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.start..rhs.span.end;
                Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span,
                }
            });

        // 逻辑或 (||)
        let logical_or = logical_and
            .clone()
            .then(
                just(Token::Or)
                    .to(BinaryOp::Or)
                    .then(logical_and)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| {
                let span = lhs.span.start..rhs.span.end;
                Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span,
                }
            });

        logical_or
    })
}
