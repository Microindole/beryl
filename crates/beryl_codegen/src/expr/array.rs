use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::generate_expr;
use beryl_syntax::ast::Expr;
use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;

/// 生成数组字面量
/// [1, 2, 3] -> 栈上分配 + 逐个存储
pub fn gen_array_literal<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<
        String,
        (
            inkwell::values::PointerValue<'ctx>,
            inkwell::types::BasicTypeEnum<'ctx>,
        ),
    >,
    elements: &[Expr],
) -> CodegenResult<BasicValueEnum<'ctx>> {
    if elements.is_empty() {
        return Err(CodegenError::UnsupportedExpression);
    }

    // 生成所有元素的值
    let mut element_values = Vec::new();
    for elem in elements {
        element_values.push(generate_expr(ctx, locals, elem)?);
    }

    // 元素类型（假设所有元素类型相同，Sema 已验证）
    let elem_type = element_values[0].get_type();
    let array_type = elem_type.array_type(elements.len() as u32);

    // 在栈上分配数组
    let array_alloca = ctx
        .builder
        .build_alloca(array_type, "array_literal")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 逐个存储元素
    for (i, value) in element_values.iter().enumerate() {
        // GEP: array_ptr, 0, i
        let indices = [
            ctx.context.i64_type().const_int(0, false),
            ctx.context.i64_type().const_int(i as u64, false),
        ];
        let elem_ptr = unsafe {
            ctx.builder
                .build_gep(
                    array_type,
                    array_alloca,
                    &indices,
                    &format!("elem_{}_ptr", i),
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
        };

        ctx.builder
            .build_store(elem_ptr, *value)
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    }

    // 加载整个数组作为值返回
    ctx.builder
        .build_load(array_type, array_alloca, "array_value")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))
}

/// 生成数组索引访问
/// arr[i] -> GEP + load (带边界检查)
pub fn gen_index_access<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<
        String,
        (
            inkwell::values::PointerValue<'ctx>,
            inkwell::types::BasicTypeEnum<'ctx>,
        ),
    >,
    array_expr: &Expr,
    index_expr: &Expr,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    // 生成数组和索引
    let array_val = generate_expr(ctx, locals, array_expr)?;
    let index_val = generate_expr(ctx, locals, index_expr)?;

    // 确保索引是整数
    let index_int = index_val.into_int_value();

    // 获取数组类型
    let array_type = array_val.get_type();

    // 数组必须是 array type
    let arr_ty = array_type.into_array_type();

    let array_size = arr_ty.len() as u64;

    // === 边界检查 ===
    // if (index < 0 || index >= size) { panic }

    // 1. index >= 0 (对于 i64，检查符号位)
    let zero = ctx.context.i64_type().const_int(0, false);
    let is_negative = ctx
        .builder
        .build_int_compare(inkwell::IntPredicate::SLT, index_int, zero, "is_negative")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 2. index < size
    let size_const = ctx.context.i64_type().const_int(array_size, false);
    let is_out_of_bounds = ctx
        .builder
        .build_int_compare(
            inkwell::IntPredicate::SGE,
            index_int,
            size_const,
            "is_out_of_bounds",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 3. is_error = is_negative || is_out_of_bounds
    let is_error = ctx
        .builder
        .build_or(is_negative, is_out_of_bounds, "is_error")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 4. 分支
    let function = ctx
        .builder
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or_else(|| CodegenError::LLVMBuildError("not in a function".to_string()))?;

    let safe_bb = ctx.context.append_basic_block(function, "index_safe");
    let panic_bb = ctx.context.append_basic_block(function, "index_panic");

    ctx.builder
        .build_conditional_branch(is_error, panic_bb, safe_bb)
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // === Panic Block ===
    ctx.builder.position_at_end(panic_bb);

    // 使用 printf + exit 内联 panic (避免链接问题)
    let i8_ptr_type = ctx
        .context
        .i8_type()
        .ptr_type(inkwell::AddressSpace::default());
    let i32_type = ctx.context.i32_type();

    // 声明/获取 printf
    let printf_fn = if let Some(func) = ctx.module.get_function("printf") {
        func
    } else {
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        ctx.module.add_function("printf", printf_type, None)
    };

    // 声明/获取 exit
    let exit_fn = if let Some(func) = ctx.module.get_function("exit") {
        func
    } else {
        let void_type = ctx.context.void_type();
        let exit_type = void_type.fn_type(&[i32_type.into()], false);
        ctx.module.add_function("exit", exit_type, None)
    };

    // 打印错误信息
    let error_msg = "Runtime Error: Array index out of bounds.\n  Index: %ld\n  Array size: %ld\n";
    let error_str = ctx
        .builder
        .build_global_string_ptr(error_msg, "panic_msg")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    ctx.builder
        .build_call(
            printf_fn,
            &[
                error_str.as_pointer_value().into(),
                index_int.into(),
                size_const.into(),
            ],
            "printf_panic",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // 调用 exit(1)
    ctx.builder
        .build_call(exit_fn, &[i32_type.const_int(1, false).into()], "exit_call")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    ctx.builder
        .build_unreachable()
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // === Safe Block ===
    ctx.builder.position_at_end(safe_bb);

    // 需要先将数组存到栈上（因为 array_val 是值）
    let array_alloca = ctx
        .builder
        .build_alloca(arr_ty, "array_temp")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    ctx.builder
        .build_store(array_alloca, array_val)
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    // GEP: array_ptr, 0, index
    let indices = [ctx.context.i64_type().const_int(0, false), index_int];
    let elem_ptr = unsafe {
        ctx.builder
            .build_gep(arr_ty, array_alloca, &indices, "elem_ptr")
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
    };

    // Load element
    let elem_type = arr_ty.get_element_type();
    ctx.builder
        .build_load(elem_type, elem_ptr, "elem_value")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))
}
/// 生成成员属性访问
/// 目前只支持数组的 .length
pub fn gen_get_property<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<
        String,
        (
            inkwell::values::PointerValue<'ctx>,
            inkwell::types::BasicTypeEnum<'ctx>,
        ),
    >,
    object: &Expr,
    name: &str,
) -> CodegenResult<BasicValueEnum<'ctx>> {
    let obj_val = generate_expr(ctx, locals, object)?;

    // 检查是否是数组类型
    if obj_val.is_array_value() {
        let array_type = obj_val.get_type().into_array_type();
        if name == "length" {
            let len = array_type.len();
            // 在 Beryl 中 length 是 int (i64)
            return Ok(ctx.context.i64_type().const_int(len as u64, false).into());
        }
    }

    // 如果是指针，可能需要检查指向的类型
    // 但在当前的 gen_variable 实现中，数组通常是被 load 为值的

    Err(CodegenError::UnsupportedExpression)
}
