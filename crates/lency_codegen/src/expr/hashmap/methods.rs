//! HashMap Code Generation
//!
//! HashMap 方法调用的代码生成

use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::CodegenValue;
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

use super::int;
use super::string;

/// Generate code for hashmap method calls via extern functions
pub fn gen_hashmap_extern_call<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    func_name: &str,
    args: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    match func_name {
        // Int Map
        "hashmap_int_new" => int::gen_new(ctx),
        "hashmap_int_insert" => int::gen_insert(ctx, locals, args),
        "hashmap_int_get" => int::gen_get(ctx, locals, args),
        "hashmap_int_contains" => int::gen_contains(ctx, locals, args),
        "hashmap_int_remove" => int::gen_remove(ctx, locals, args),
        "hashmap_int_len" => int::gen_len(ctx, locals, args),

        // String Map
        "hashmap_string_new" => string::gen_new(ctx),
        "hashmap_string_insert" => string::gen_insert(ctx, locals, args),
        "hashmap_string_get" => string::gen_get(ctx, locals, args),
        "hashmap_string_contains" => string::gen_contains(ctx, locals, args),
        "hashmap_string_remove" => string::gen_remove(ctx, locals, args),
        "hashmap_string_len" => string::gen_len(ctx, locals, args),

        _ => Err(CodegenError::FunctionNotFound(func_name.to_string())),
    }
}
