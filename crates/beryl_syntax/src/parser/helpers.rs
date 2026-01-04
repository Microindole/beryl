//! Parser Helper Functions
//!
//! 辅助解析函数：标识符、类型、字段等

use crate::ast::{Field, Type};
use crate::lexer::Token;
use chumsky::prelude::*;

pub type ParserError = Simple<Token>;

/// 解析标识符
pub fn ident_parser() -> impl Parser<Token, String, Error = ParserError> + Clone {
    select! { Token::Ident(ident) => ident }
}

/// 解析类型
pub fn type_parser() -> impl Parser<Token, Type, Error = ParserError> + Clone {
    // 完全避免递归和clone问题的版本

    // 基础类型
    let basic = select! {
        Token::TypeInt => Type::Int,
        Token::TypeFloat => Type::Float,
        Token::TypeString => Type::String,
        Token::TypeBool => Type::Bool,
        Token::TypeVoid => Type::Void,
    };

    // 标识符（可能带泛型参数）
    let ident_with_generics = ident_parser()
        .then(
            ident_parser()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .delimited_by(just(Token::Lt), just(Token::Gt))
                .or_not(),
        )
        .map(|(name, args)| {
            if let Some(params) = args {
                Type::Generic(name, params.into_iter().map(Type::Struct).collect())
            } else {
                Type::Struct(name)
            }
        });

    // Vec<ident>
    let vec_simple = just(Token::Vec)
        .ignore_then(just(Token::Lt))
        .ignore_then(ident_parser())
        .then_ignore(just(Token::Gt))
        .map(|name| Type::Vec(Box::new(Type::Struct(name))));

    // Vec<基础类型>
    let vec_basic = just(Token::Vec)
        .ignore_then(just(Token::Lt))
        .ignore_then(basic.clone())
        .then_ignore(just(Token::Gt))
        .map(|ty| Type::Vec(Box::new(ty)));

    // [N]ident
    let array_ident = just(Token::LBracket)
        .ignore_then(select! { Token::Int(n) => n as usize })
        .then_ignore(just(Token::RBracket))
        .then(ident_parser())
        .map(|(size, name)| Type::Array {
            element_type: Box::new(Type::Struct(name)),
            size,
        });

    // [N]基础类型
    let array_basic = just(Token::LBracket)
        .ignore_then(select! { Token::Int(n) => n as usize })
        .then_ignore(just(Token::RBracket))
        .then(basic.clone())
        .map(|(size, ty)| Type::Array {
            element_type: Box::new(ty),
            size,
        });

    // 使用choice来避免or chain
    let type_without_nullable = choice((
        vec_basic,
        vec_simple,
        array_basic,
        array_ident,
        ident_with_generics,
        basic,
    ));

    // 可空类型
    type_without_nullable
        .then(just(Token::Question).or_not())
        .map(|(ty, q)| {
            if q.is_some() {
                Type::Nullable(Box::new(ty))
            } else {
                ty
            }
        })
}

/// 解析字段
pub fn field_parser() -> impl Parser<Token, Field, Error = ParserError> + Clone {
    type_parser()
        .then(ident_parser())
        .then_ignore(just(Token::Semicolon).or_not())
        .map(|(ty, name)| Field { name, ty })
}

/// 解析泛型参数列表: <T, U>
/// 返回空Vec如果没有泛型参数
pub fn generic_params_parser() -> impl Parser<Token, Vec<String>, Error = ParserError> + Clone {
    ident_parser()
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .delimited_by(just(Token::Lt), just(Token::Gt))
        .or_not()
        .map(|opt| opt.unwrap_or_default())
}
