//! Vec Literal Code Generation

use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use beryl_syntax::ast::{Expr, Type};
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use std::collections::HashMap;

/// Generate code for vec![...] literals
pub fn gen_vec_literal<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    elements: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    // 1. Declare beryl_vec_new if not already declared
    let vec_new_fn = get_or_declare_vec_new(ctx)?;

    // 2. Declare beryl_vec_push if not already declared
    let vec_push_fn = get_or_declare_vec_push(ctx)?;

    // 3. Call beryl_vec_new(capacity)
    let capacity = ctx
        .context
        .i64_type()
        .const_int(elements.len() as u64, false);
    let vec_ptr = ctx
        .builder
        .build_call(vec_new_fn, &[capacity.into()], "vec")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
        .try_as_basic_value()
        .left()
        .ok_or_else(|| CodegenError::LLVMBuildError("vec_new returned void".to_string()))?;

    // 4. For each element, call beryl_vec_push(vec, element)
    for elem in elements {
        let elem_val = generate_expr(ctx, locals, elem)?;

        // Ensure element is i64
        let elem_i64 = match elem_val.value {
            BasicValueEnum::IntValue(iv) => {
                // If it's not i64, cast it
                if iv.get_type() == ctx.context.i64_type() {
                    iv
                } else {
                    ctx.builder
                        .build_int_cast(iv, ctx.context.i64_type(), "cast_to_i64")
                        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                }
            }
            _ => {
                return Err(CodegenError::TypeMismatch);
            }
        };

        ctx.builder
            .build_call(vec_push_fn, &[vec_ptr.into(), elem_i64.into()], "")
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    }

    // 5. Return the vec pointer
    Ok(CodegenValue {
        value: vec_ptr,
        ty: Type::Vec,
    })
}

/// Get or declare beryl_vec_new function
fn get_or_declare_vec_new<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> CodegenResult<inkwell::values::FunctionValue<'ctx>> {
    if let Some(func) = ctx.module.get_function("beryl_vec_new") {
        return Ok(func);
    }

    // declare i8* @beryl_vec_new(i64)
    let i64_type = ctx.context.i64_type();
    let vec_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let fn_type = vec_ptr_type.fn_type(&[i64_type.into()], false);

    Ok(ctx.module.add_function("beryl_vec_new", fn_type, None))
}

/// Get or declare beryl_vec_push function
fn get_or_declare_vec_push<'ctx>(
    ctx: &CodegenContext<'ctx>,
) -> CodegenResult<inkwell::values::FunctionValue<'ctx>> {
    if let Some(func) = ctx.module.get_function("beryl_vec_push") {
        return Ok(func);
    }

    // declare void @beryl_vec_push(i8*, i64)
    let i64_type = ctx.context.i64_type();
    let vec_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let fn_type = ctx
        .context
        .void_type()
        .fn_type(&[vec_ptr_type.into(), i64_type.into()], false);

    Ok(ctx.module.add_function("beryl_vec_push", fn_type, None))
}
