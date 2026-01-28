use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use inkwell::AddressSpace;
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

pub fn gen_new<'ctx>(ctx: &CodegenContext<'ctx>) -> CodegenResult<CodegenValue<'ctx>> {
    let fn_type = ctx
        .context
        .i8_type()
        .ptr_type(AddressSpace::default())
        .fn_type(&[], false);
    let func = ctx
        .module
        .get_function("lency_hashmap_string_new")
        .unwrap_or_else(|| {
            ctx.module
                .add_function("lency_hashmap_string_new", fn_type, None)
        });
    let call = ctx
        .builder
        .build_call(func, &[], "map")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    let ptr = call.try_as_basic_value().left().unwrap();
    let ptr_as_int = ctx
        .builder
        .build_ptr_to_int(ptr.into_pointer_value(), ctx.context.i64_type(), "map_int")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    Ok(CodegenValue {
        value: ptr_as_int.into(),
        ty: Type::Int,
    })
}

pub fn gen_insert<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    args: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    let fn_type = ctx.context.void_type().fn_type(
        &[
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            ctx.context.i64_type().into(),
        ],
        false,
    );
    let func = ctx
        .module
        .get_function("lency_hashmap_string_insert")
        .unwrap_or_else(|| {
            ctx.module
                .add_function("lency_hashmap_string_insert", fn_type, None)
        });
    let map_val = generate_expr(ctx, locals, &args[0])?;
    let key_val = generate_expr(ctx, locals, &args[1])?;
    let value_val = generate_expr(ctx, locals, &args[2])?;
    let map_ptr = ctx
        .builder
        .build_int_to_ptr(
            map_val.value.into_int_value(),
            ctx.context.i8_type().ptr_type(AddressSpace::default()),
            "map_ptr",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    ctx.builder
        .build_call(
            func,
            &[map_ptr.into(), key_val.value.into(), value_val.value.into()],
            "",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    Ok(CodegenValue {
        value: ctx.context.i64_type().const_zero().into(),
        ty: Type::Void,
    })
}

pub fn gen_get<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    args: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    let fn_type = ctx.context.i64_type().fn_type(
        &[
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
        ],
        false,
    );
    let func = ctx
        .module
        .get_function("lency_hashmap_string_get")
        .unwrap_or_else(|| {
            ctx.module
                .add_function("lency_hashmap_string_get", fn_type, None)
        });
    let map_val = generate_expr(ctx, locals, &args[0])?;
    let key_val = generate_expr(ctx, locals, &args[1])?;
    let map_ptr = ctx
        .builder
        .build_int_to_ptr(
            map_val.value.into_int_value(),
            ctx.context.i8_type().ptr_type(AddressSpace::default()),
            "map_ptr",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    let call = ctx
        .builder
        .build_call(func, &[map_ptr.into(), key_val.value.into()], "get_res")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    Ok(CodegenValue {
        value: call.try_as_basic_value().left().unwrap(),
        ty: Type::Int,
    })
}

pub fn gen_contains<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    args: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    let fn_type = ctx.context.bool_type().fn_type(
        &[
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
        ],
        false,
    );
    let func = ctx
        .module
        .get_function("lency_hashmap_string_contains")
        .unwrap_or_else(|| {
            ctx.module
                .add_function("lency_hashmap_string_contains", fn_type, None)
        });
    let map_val = generate_expr(ctx, locals, &args[0])?;
    let key_val = generate_expr(ctx, locals, &args[1])?;
    let map_ptr = ctx
        .builder
        .build_int_to_ptr(
            map_val.value.into_int_value(),
            ctx.context.i8_type().ptr_type(AddressSpace::default()),
            "map_ptr",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    let call = ctx
        .builder
        .build_call(
            func,
            &[map_ptr.into(), key_val.value.into()],
            "contains_res",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    Ok(CodegenValue {
        value: call.try_as_basic_value().left().unwrap(),
        ty: Type::Bool,
    })
}

pub fn gen_remove<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    args: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    let fn_type = ctx.context.bool_type().fn_type(
        &[
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            ctx.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
        ],
        false,
    );
    let func = ctx
        .module
        .get_function("lency_hashmap_string_remove")
        .unwrap_or_else(|| {
            ctx.module
                .add_function("lency_hashmap_string_remove", fn_type, None)
        });
    let map_val = generate_expr(ctx, locals, &args[0])?;
    let key_val = generate_expr(ctx, locals, &args[1])?;
    let map_ptr = ctx
        .builder
        .build_int_to_ptr(
            map_val.value.into_int_value(),
            ctx.context.i8_type().ptr_type(AddressSpace::default()),
            "map_ptr",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    let call = ctx
        .builder
        .build_call(func, &[map_ptr.into(), key_val.value.into()], "remove_res")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    Ok(CodegenValue {
        value: call.try_as_basic_value().left().unwrap(),
        ty: Type::Bool,
    })
}

pub fn gen_len<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, Type)>,
    args: &[Expr],
) -> CodegenResult<CodegenValue<'ctx>> {
    let fn_type = ctx.context.i64_type().fn_type(
        &[ctx
            .context
            .i8_type()
            .ptr_type(AddressSpace::default())
            .into()],
        false,
    );
    let func = ctx
        .module
        .get_function("lency_hashmap_string_len")
        .unwrap_or_else(|| {
            ctx.module
                .add_function("lency_hashmap_string_len", fn_type, None)
        });
    let map_val = generate_expr(ctx, locals, &args[0])?;
    let map_ptr = ctx
        .builder
        .build_int_to_ptr(
            map_val.value.into_int_value(),
            ctx.context.i8_type().ptr_type(AddressSpace::default()),
            "map_ptr",
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    let call = ctx
        .builder
        .build_call(func, &[map_ptr.into()], "len_res")
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
    Ok(CodegenValue {
        value: call.try_as_basic_value().left().unwrap(),
        ty: Type::Int,
    })
}
