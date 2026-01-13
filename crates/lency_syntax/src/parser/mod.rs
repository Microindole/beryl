//! Parser Module
//!
//! 模块化的 Parser 实现，遵循开闭原则

pub mod decl;
pub mod expr;
pub mod helpers;
pub mod pattern;
pub mod stmt;

use crate::ast::Program;
use crate::lexer::Token;
use chumsky::prelude::*;

pub type ParserError = Simple<Token>;

/// 主入口：解析整个程序
pub fn program_parser() -> impl Parser<Token, Program, Error = ParserError> {
    decl::decl_parser()
        .repeated()
        .map(|decls| Program { decls })
        .then_ignore(end())
}

/// 辅助函数：解析源码字符串
pub fn parse(code: &str) -> Result<Program, Vec<ParserError>> {
    use logos::Logos;
    let tokens: Vec<Token> = Token::lexer(code)
        .spanned()
        .map(|(tok, _span)| tok.unwrap_or(Token::Error))
        .collect();

    program_parser().parse(tokens)
}
