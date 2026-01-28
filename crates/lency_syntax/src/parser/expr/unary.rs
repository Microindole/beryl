use crate::ast::*;
use crate::lexer::Token;
use chumsky::prelude::*;

use super::ParserError;

pub fn parser(
    postfix: impl Parser<Token, Expr, Error = ParserError> + Clone,
) -> impl Parser<Token, Expr, Error = ParserError> + Clone {
    just(Token::Minus)
        .to(UnaryOp::Neg)
        .or(just(Token::Bang).to(UnaryOp::Not))
        .map_with_span(|op, span| (op, span))
        .repeated()
        .then(postfix)
        .foldr(|(op, span), rhs| {
            let new_span = span.start..rhs.span.end;
            Expr {
                kind: ExprKind::Unary(op, Box::new(rhs)),
                span: new_span,
            }
        })
}
