use super::ffi;
use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use crate::types::ToLLVMType;
use inkwell::AddressSpace;
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// 生成 write_file("path", "content") 调用
/// 返回 Result<void, Error> (即 void!)
pub fn gen_write_file<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    path_expr: &Expr,
    content_expr: &Expr,
) -> CodegenResult<CodegenValue<'ctx>> {
    // 1. 生成参数
    let path_val = generate_expr(ctx, locals, path_expr)?;
    let path_ptr = path_val.value.into_pointer_value();

    let content_val = generate_expr(ctx, locals, content_expr)?;
    let content_ptr = content_val.value.into_pointer_value();

    // 2. 声明 FFI 函数
    let file_open_fn = ffi::get_or_declare_open(ctx);
    let file_write_fn = ffi::get_or_declare_write(ctx);
    let file_close_fn = ffi::get_or_declare_close(ctx);

    let i64_type = ctx.context.i64_type();
    let i8_ptr_type = ctx.context.i8_type().ptr_type(AddressSpace::default());

    // 3. 调用 lency_file_open(path, 1) 写模式
    let mode_write = i64_type.const_int(1, false);
    let file_handle = ctx
        .builder
        .build_call(
            file_open_fn,
            &[path_ptr.into(), mode_write.into()],
            "write_handle",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
        .try_as_basic_value()
        .left()
        .ok_or(CodegenError::LLVMBuildError(
            "file_open returned void".to_string(),
        ))?
        .into_pointer_value();

    // 4. 检查 handle 是否为 null
    let null_ptr = i8_ptr_type.const_null();
    let is_null = ctx
        .builder
        .build_int_compare(inkwell::IntPredicate::EQ, file_handle, null_ptr, "is_null")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    let current_block = ctx
        .builder
        .get_insert_block()
        .ok_or(CodegenError::LLVMBuildError("No insert block".to_string()))?;
    let parent_func = current_block
        .get_parent()
        .ok_or(CodegenError::LLVMBuildError(
            "No parent function".to_string(),
        ))?;

    let success_block = ctx.context.append_basic_block(parent_func, "write_success");
    let error_block = ctx.context.append_basic_block(parent_func, "write_error");
    let merge_block = ctx.context.append_basic_block(parent_func, "write_merge");

    ctx.builder
        .build_conditional_branch(is_null, error_block, success_block)
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 5. Success path: 写入文件
    ctx.builder.position_at_end(success_block);

    let _bytes_written = ctx
        .builder
        .build_call(
            file_write_fn,
            &[file_handle.into(), content_ptr.into()],
            "bytes_written",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    ctx.builder
        .build_call(file_close_fn, &[file_handle.into()], "")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 构造 Ok(void) Result - 使用已注册的类型
    let result_ty = Type::Result {
        ok_type: Box::new(Type::Void),
        err_type: Box::new(Type::Struct("Error".to_string())),
    };

    // 获取已注册的 Result struct type
    let mangled_name = lency_monomorph::mangling::mangle_type(&result_ty);
    let struct_type = *ctx
        .struct_types
        .get(&mangled_name)
        .ok_or_else(|| CodegenError::UndefinedStructType(mangled_name.clone()))?;

    let malloc_fn = ffi::get_or_declare_malloc(ctx)
        .ok_or(CodegenError::LLVMBuildError("malloc not found".to_string()))?;
    let result_size = struct_type.size_of().ok_or(CodegenError::LLVMBuildError(
        "Failed to get Result size".to_string(),
    ))?;

    let ok_result_raw = ctx
        .builder
        .build_call(malloc_fn, &[result_size.into()], "ok_result")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
        .try_as_basic_value()
        .left()
        .ok_or(CodegenError::LLVMBuildError(
            "malloc returned void".to_string(),
        ))?
        .into_pointer_value();

    let result_ptr_type = result_ty.to_llvm_type(ctx)?.into_pointer_type();
    let ok_result_ptr = ctx
        .builder
        .build_pointer_cast(ok_result_raw, result_ptr_type, "ok_result_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    let is_ok_ptr = ctx
        .builder
        .build_struct_gep(struct_type, ok_result_ptr, 0, "is_ok_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    ctx.builder
        .build_store(is_ok_ptr, ctx.context.bool_type().const_int(1, false))
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    ctx.builder
        .build_unconditional_branch(merge_block)
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 6. Error path: 构造 Err(Error)
    ctx.builder.position_at_end(error_block);

    let err_result_raw = ctx
        .builder
        .build_call(malloc_fn, &[result_size.into()], "err_result")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
        .try_as_basic_value()
        .left()
        .ok_or(CodegenError::LLVMBuildError(
            "malloc returned void".to_string(),
        ))?
        .into_pointer_value();

    let err_result_ptr = ctx
        .builder
        .build_pointer_cast(err_result_raw, result_ptr_type, "err_result_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    let is_ok_err_ptr = ctx
        .builder
        .build_struct_gep(struct_type, err_result_ptr, 0, "is_ok_err_ptr")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    ctx.builder
        .build_store(is_ok_err_ptr, ctx.context.bool_type().const_int(0, false))
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    ctx.builder
        .build_unconditional_branch(merge_block)
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 7. Merge block: 使用 PHI 节点选择结果
    ctx.builder.position_at_end(merge_block);

    let phi = ctx
        .builder
        .build_phi(result_ptr_type, "write_file_result")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    phi.add_incoming(&[
        (&ok_result_ptr, success_block),
        (&err_result_ptr, error_block),
    ]);

    Ok(CodegenValue {
        value: phi.as_basic_value(),
        ty: result_ty,
    })
}
