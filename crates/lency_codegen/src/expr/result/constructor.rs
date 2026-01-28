use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};

use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// 生成 Ok(val) 构造器
pub fn gen_ok<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, lency_syntax::ast::Type)>,
    inner: &Expr,
) -> CodegenResult<CodegenValue<'ctx>> {
    // 1. 生成内部值
    let val_wrapper = generate_expr(ctx, locals, inner)?;
    let ok_val = val_wrapper.value;
    let ok_ty = val_wrapper.ty;

    // 2. 构造 Result 类型: Result<T, Error>
    let result_ty = Type::Result {
        ok_type: Box::new(ok_ty.clone()),
        err_type: Box::new(Type::Struct("Error".to_string())),
    };

    // 3. 获取已注册的 Result struct type
    let mangled_name = lency_monomorph::mangling::mangle_type(&result_ty);
    let struct_type = *ctx
        .struct_types
        .get(&mangled_name)
        .ok_or_else(|| crate::error::CodegenError::UndefinedStructType(mangled_name.clone()))?;

    // 获取 Result pointer type
    let result_ptr_type = struct_type.ptr_type(inkwell::AddressSpace::default());

    // 4. Malloc
    let size = struct_type.size_of().ok_or(CodegenError::LLVMBuildError(
        "Failed to get size of Result type".to_string(),
    ))?;

    let malloc = ctx
        .module
        .get_function("malloc")
        .ok_or(CodegenError::LLVMBuildError("malloc not found".to_string()))?;

    let malloc_call = ctx
        .builder
        .build_call(malloc, &[size.into()], "malloc_result")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    let raw_ptr = malloc_call
        .try_as_basic_value()
        .left()
        .ok_or(CodegenError::LLVMBuildError(
            "malloc returned void".to_string(),
        ))?
        .into_pointer_value();

    // 5. Cast and Store
    let result_ptr = ctx
        .builder
        .build_pointer_cast(raw_ptr, result_ptr_type, "result_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // Store is_ok = true (1)
    let is_ok_ptr = ctx
        .builder
        .build_struct_gep(struct_type, result_ptr, 0, "is_ok_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    ctx.builder
        .build_store(is_ok_ptr, ctx.context.bool_type().const_int(1, false))
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // Store ok_value (index 1)
    if !matches!(ok_ty, Type::Void) {
        let val_ptr = ctx
            .builder
            .build_struct_gep(struct_type, result_ptr, 1, "ok_val_ptr")
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
        ctx.builder
            .build_store(val_ptr, ok_val)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    }

    Ok(CodegenValue {
        value: result_ptr.into(),
        ty: result_ty,
    })
}

/// 生成 Err(msg) 构造器
pub fn gen_err<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, lency_syntax::ast::Type)>,
    inner: &Expr,
) -> CodegenResult<CodegenValue<'ctx>> {
    // 1. 生成内部值 (Error object or fields)
    let val_wrapper = generate_expr(ctx, locals, inner)?;
    let err_val = val_wrapper.value;
    // Assume inner is Error struct pointer (Type::Struct("Error"))

    // 2. 构造 Result 类型: Result<void, Error>
    let result_ty = Type::Result {
        ok_type: Box::new(Type::Void),
        err_type: Box::new(Type::Struct("Error".to_string())),
    };

    // 3. 获取已注册的 Result struct type
    let mangled_name = lency_monomorph::mangling::mangle_type(&result_ty);
    let struct_type = *ctx
        .struct_types
        .get(&mangled_name)
        .ok_or_else(|| crate::error::CodegenError::UndefinedStructType(mangled_name.clone()))?;

    // 获取 Result pointer type
    let result_ptr_type = struct_type.ptr_type(inkwell::AddressSpace::default());

    // 4. Malloc
    let size = struct_type.size_of().ok_or(CodegenError::LLVMBuildError(
        "Failed to get size of Result type".to_string(),
    ))?;

    let malloc = ctx
        .module
        .get_function("malloc")
        .ok_or(CodegenError::LLVMBuildError("malloc not found".to_string()))?;

    let malloc_call = ctx
        .builder
        .build_call(malloc, &[size.into()], "malloc_result_err")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    let raw_ptr = malloc_call
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_pointer_value();

    // 5. Store
    let result_ptr = ctx
        .builder
        .build_pointer_cast(raw_ptr, result_ptr_type, "result_err_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // Store is_ok = false (0)
    let is_ok_ptr = ctx
        .builder
        .build_struct_gep(struct_type, result_ptr, 0, "is_ok_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    ctx.builder
        .build_store(is_ok_ptr, ctx.context.bool_type().const_int(0, false))
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // Store err_value (index 1) -- because ok_val is void and skipped
    let err_ptr = ctx
        .builder
        .build_struct_gep(struct_type, result_ptr, 1, "err_val_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    ctx.builder
        .build_store(err_ptr, err_val)
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    Ok(CodegenValue {
        value: result_ptr.into(),
        ty: result_ty,
    })
}
