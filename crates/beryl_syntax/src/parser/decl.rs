//! Declaration Parser
//!
//! 声明解析：函数、类

use super::helpers::{field_parser, generic_params_parser, ident_parser, type_parser};

use super::stmt::stmt_parser;
use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

pub type ParserError = Simple<Token>;

/// 解析声明 (公共接口)
pub fn decl_parser() -> impl Parser<Token, Decl, Error = ParserError> {
    let stmt = stmt_parser().boxed();
    recursive(|_decl| {
        // 函数声明: int add(int a, int b) { ... }
        // 泛型函数: T identity<T>(T x) { ... }
        let func = type_parser()
            .then(ident_parser())
            .then(generic_params_parser()) // 解析 <T, U>
            .then(
                type_parser()
                    .then(ident_parser())
                    .map(|(ty, name)| Param { name, ty })
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .then(
                stmt.clone()
                    .repeated()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with_span(
                |((((return_type, name), generic_params), params), body), span| Decl::Function {
                    span,
                    name,
                    generic_params,
                    params,
                    return_type,
                    body,
                },
            );

        // 外部函数声明: extern int print(int n);
        let extern_decl = just(Token::Extern)
            .ignore_then(type_parser())
            .then(ident_parser())
            .then(generic_params_parser()) // 解析 <T>
            .then(
                type_parser()
                    .then(ident_parser())
                    .map(|(ty, name)| Param { name, ty })
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .delimited_by(just(Token::LParen), just(Token::RParen)),
            )
            .then_ignore(just(Token::Semicolon))
            .map_with_span(|(((return_type, name), generic_params), params), span| {
                Decl::ExternFunction {
                    span,
                    name,
                    generic_params,
                    params,
                    return_type,
                }
            });

        // 结构体声明: struct Point { int x int y }
        // 泛型结构体: struct Box<T> { T value }
        let struct_decl = just(Token::Struct)
            .ignore_then(ident_parser())
            .then(generic_params_parser()) // 解析 <T, U>
            .then(
                field_parser()
                    .repeated()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with_span(|((name, generic_params), fields), span| Decl::Struct {
                span,
                name,
                generic_params,
                fields,
            });

        // impl 块: impl Point { ... }
        // 泛型impl: impl<T> Box<T> { ... }
        let impl_decl = just(Token::Impl)
            .ignore_then(generic_params_parser()) // 解析 <T>
            .then(ident_parser())
            .then(
                func.clone()
                    .repeated()
                    .delimited_by(just(Token::LBrace), just(Token::RBrace)),
            )
            .map_with_span(|((generic_params, type_name), methods), span| Decl::Impl {
                span,
                type_name,
                generic_params,
                methods,
            });

        choice((struct_decl, impl_decl, extern_decl, func))
    })
}
