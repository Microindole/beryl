//! Binary Operation Code Generation
//!
//! 二元运算代码生成

use beryl_syntax::ast::{BinaryOp, Expr};
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;

use crate::context::CodegenContext;
use crate::error::CodegenResult;

use super::generate_expr;

pub mod arithmetic;
pub mod comparison;
pub mod logical;

use arithmetic::{gen_add, gen_div, gen_mod, gen_mul, gen_sub};
use comparison::{gen_eq, gen_geq, gen_gt, gen_leq, gen_lt, gen_neq};
use logical::{gen_and, gen_or};

/// 生成二元运算代码
pub(super) fn gen_binary<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<
        String,
        (
            inkwell::values::PointerValue<'ctx>,
            inkwell::types::BasicTypeEnum<'ctx>,
        ),
    >,
    left: &Expr,
    op: &BinaryOp,
    right: &Expr,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    let lhs = generate_expr(ctx, locals, left)?;
    let rhs = generate_expr(ctx, locals, right)?;

    match op {
        BinaryOp::Add => gen_add(ctx, lhs, rhs),
        BinaryOp::Sub => gen_sub(ctx, lhs, rhs),
        BinaryOp::Mul => gen_mul(ctx, lhs, rhs),
        BinaryOp::Div => gen_div(ctx, lhs, rhs),
        BinaryOp::Mod => gen_mod(ctx, lhs, rhs),
        BinaryOp::Eq => gen_eq(ctx, lhs, rhs),
        BinaryOp::Neq => gen_neq(ctx, lhs, rhs),
        BinaryOp::Lt => gen_lt(ctx, lhs, rhs),
        BinaryOp::Gt => gen_gt(ctx, lhs, rhs),
        BinaryOp::Leq => gen_leq(ctx, lhs, rhs),
        BinaryOp::Geq => gen_geq(ctx, lhs, rhs),
        BinaryOp::And => gen_and(ctx, lhs, rhs),
        BinaryOp::Or => gen_or(ctx, lhs, rhs),
    }
}
