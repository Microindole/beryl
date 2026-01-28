use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};

use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// Result 内置方法实现
///
/// 直接读取 Result 结构的内部字段，无需 match 语法
/// Result 内存布局:
///   index 0: is_ok (bool)
///   index 1: ok_value (T)  [如果 T != void]
///   index 2: err_value (E) [或 index 1 如果 T == void]
pub fn gen_result_builtin_method<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    result_ptr: inkwell::values::PointerValue<'ctx>,
    method_name: &str,
    args: &[Expr],
    ok_type: &Type,
    _err_type: &Type,
) -> CodegenResult<Option<CodegenValue<'ctx>>> {
    // 获取 Result struct type
    let result_ty = Type::Result {
        ok_type: Box::new(ok_type.clone()),
        err_type: Box::new(Type::Struct("Error".to_string())),
    };
    let mangled_name = lency_monomorph::mangling::mangle_type(&result_ty);

    let struct_type = match ctx.struct_types.get(&mangled_name) {
        Some(st) => *st,
        None => return Ok(None), // 无法找到类型，fallback 到编译方法
    };

    match method_name {
        "is_ok" => {
            // 读取 index 0 (is_ok 字段)
            let is_ok_ptr = ctx
                .builder
                .build_struct_gep(struct_type, result_ptr, 0, "is_ok_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let is_ok = ctx
                .builder
                .build_load(ctx.context.bool_type(), is_ok_ptr, "is_ok")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            Ok(Some(CodegenValue {
                value: is_ok,
                ty: Type::Bool,
            }))
        }
        "is_err" => {
            // 读取 index 0 (is_ok 字段) 并取反
            let is_ok_ptr = ctx
                .builder
                .build_struct_gep(struct_type, result_ptr, 0, "is_ok_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let is_ok = ctx
                .builder
                .build_load(ctx.context.bool_type(), is_ok_ptr, "is_ok")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_int_value();

            // 取反: is_err = !is_ok
            let is_err = ctx
                .builder
                .build_not(is_ok, "is_err")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            Ok(Some(CodegenValue {
                value: is_err.into(),
                ty: Type::Bool,
            }))
        }
        "unwrap_or" => {
            // unwrap_or(default) -> 如果 is_ok 返回 ok_val，否则返回 default
            if args.len() != 1 {
                return Ok(None); // 参数错误，fallback
            }

            // 生成 default 值
            let default_val = generate_expr(ctx, locals, &args[0])?;

            // 读取 is_ok
            let is_ok_ptr = ctx
                .builder
                .build_struct_gep(struct_type, result_ptr, 0, "is_ok_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let is_ok = ctx
                .builder
                .build_load(ctx.context.bool_type(), is_ok_ptr, "is_ok")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_int_value();

            // 读取 ok_value (index 1)
            let ok_llvm_type = crate::types::ToLLVMType::to_llvm_type(ok_type, ctx)?;
            let ok_val_ptr = ctx
                .builder
                .build_struct_gep(struct_type, result_ptr, 1, "ok_val_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let ok_val = ctx
                .builder
                .build_load(ok_llvm_type, ok_val_ptr, "ok_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // 使用 select 指令: is_ok ? ok_val : default
            let result = ctx
                .builder
                .build_select(is_ok, ok_val, default_val.value, "unwrap_or_result")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            Ok(Some(CodegenValue {
                value: result,
                ty: ok_type.clone(),
            }))
        }
        _ => Ok(None), // 未知方法，fallback 到编译方法
    }
}
