//! Intrinsic Functions Code Generation
//!
//! 内置函数代码生成：print
//! 文件 I/O 函数已移至 file_io 模块

use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use beryl_syntax::ast::{Expr, Type};
use inkwell::AddressSpace;
use std::collections::HashMap;

// 重新导出 file_io 模块的函数
pub use super::file_io::{gen_read_file, gen_write_file};

/// 生成 Print 内建函数调用
pub fn gen_print<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, beryl_syntax::ast::Type)>,
    arg: &Expr,
) -> CodegenResult<CodegenValue<'ctx>> {
    let arg_val = generate_expr(ctx, locals, arg)?;

    let i8_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = ctx.context.i64_type();

    match arg_val.ty {
        Type::Int => {
            // 使用 printf("%lld\n", value) 打印整数
            let printf_fn = ctx.module.get_function("printf").unwrap_or_else(|| {
                let fn_type = i64_type.fn_type(&[i8_ptr_type.into()], true);
                ctx.module.add_function("printf", fn_type, None)
            });

            let format_str = ctx
                .builder
                .build_global_string_ptr("%lld\n", "int_fmt")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            ctx.builder
                .build_call(
                    printf_fn,
                    &[format_str.as_pointer_value().into(), arg_val.value.into()],
                    "print_int",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
        }
        Type::Float => {
            let printf_fn = ctx.module.get_function("printf").unwrap_or_else(|| {
                let fn_type = i64_type.fn_type(&[i8_ptr_type.into()], true);
                ctx.module.add_function("printf", fn_type, None)
            });

            let format_str = ctx
                .builder
                .build_global_string_ptr("%f\n", "float_fmt")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            ctx.builder
                .build_call(
                    printf_fn,
                    &[format_str.as_pointer_value().into(), arg_val.value.into()],
                    "print_float",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
        }
        Type::Bool => {
            let printf_fn = ctx.module.get_function("printf").unwrap_or_else(|| {
                let fn_type = i64_type.fn_type(&[i8_ptr_type.into()], true);
                ctx.module.add_function("printf", fn_type, None)
            });

            let format_str = ctx
                .builder
                .build_global_string_ptr("%s\n", "bool_fmt")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            let true_str = ctx
                .builder
                .build_global_string_ptr("true", "true_str")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let false_str = ctx
                .builder
                .build_global_string_ptr("false", "false_str")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            let bool_val = arg_val.value.into_int_value();
            let str_val = ctx
                .builder
                .build_select(
                    bool_val,
                    true_str.as_pointer_value(),
                    false_str.as_pointer_value(),
                    "bool_str",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            ctx.builder
                .build_call(
                    printf_fn,
                    &[format_str.as_pointer_value().into(), str_val.into()],
                    "print_bool",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
        }
        Type::String => {
            let puts_fn = ctx.module.get_function("puts").unwrap_or_else(|| {
                let fn_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
                ctx.module.add_function("puts", fn_type, None)
            });

            ctx.builder
                .build_call(puts_fn, &[arg_val.value.into()], "print_str")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
        }
        _ => {
            // 其他类型尚不支持直接打印
            return Err(CodegenError::LLVMBuildError(format!(
                "print() not implemented for type: {:?}",
                arg_val.ty
            )));
        }
    }

    Ok(CodegenValue {
        value: ctx.context.i64_type().const_int(0, false).into(),
        ty: Type::Void,
    })
}
