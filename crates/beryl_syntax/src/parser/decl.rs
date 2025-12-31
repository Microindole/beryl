//! Declaration Parser
//!
//! 声明解析：函数、类

use super::helpers::{field_parser, ident_parser, type_parser};
use super::stmt::stmt_parser;
use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

pub type ParserError = Simple<Token>;

/// 解析声明 (公共接口)
pub fn decl_parser() -> impl Parser<Token, Decl, Error = ParserError> {
    recursive(|_decl| {
        // 函数声明: int add(int a, int b) { ... }
        let func = type_parser()
            .then(ident_parser())
            .then(
                type_parser()
                    .then(ident_parser())
                    .map(|(ty, name)| Param { name, ty })
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .then(
                stmt_parser()
                    .repeated()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with_span(
                |(((return_type, name), params), body), span| Decl::Function {
                    span,
                    name,
                    params,
                    return_type,
                    body,
                },
            );

        // 类声明: class User { ... }
        let class_decl = just(Token::Class)
            .ignore_then(ident_parser())
            .then(
                ident_parser()
                    .separated_by(just(Token::Comma))
                    .delimited_by(just(Token::Lt), just(Token::Gt))
                    .or_not()
                    .map(|g| g.unwrap_or_default()),
            )
            .then(
                field_parser()
                    .repeated()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with_span(|((name, generics), fields), span| Decl::Class {
                span,
                name,
                generics,
                fields,
                methods: vec![],
            });

        class_decl.or(func)
    })
}
