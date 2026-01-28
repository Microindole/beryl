use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use inkwell::values::PointerValue;
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// 生成成员指针（LValue）
/// 用于赋值或读取
pub fn gen_struct_member_ptr<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, lency_syntax::ast::Type)>,
    object_expr: &Expr,
    field_name: &str,
    line: u32,
) -> CodegenResult<PointerValue<'ctx>> {
    // 1. 计算对象表达式
    let object_val = generate_expr(ctx, locals, object_expr)?;

    // 2. 必须是指针类型（结构体是通过指针传递的）
    if !object_val.value.is_pointer_value() {
        return Err(CodegenError::UnsupportedType(
            "Field access on non-pointer value [LValue]".to_string(),
        ));
    }
    let ptr_val = object_val.value.into_pointer_value();

    // 运行时 Null 检查
    if let Some(panic_func) = ctx.panic_func {
        crate::runtime::gen_null_check(ctx.context, &ctx.builder, panic_func, ptr_val, line);
    }

    // 3. 获取结构体名称和 LLVM 类型
    let struct_name = match &object_val.ty {
        Type::Struct(name) => name,

        _ => {
            return Err(CodegenError::UnsupportedType(format!(
                "Field access on non-struct type: {:?}",
                object_val.ty
            )))
        }
    };

    let struct_type = ctx.struct_types.get(struct_name).ok_or_else(|| {
        CodegenError::UnsupportedType(format!("Unknown struct '{}'", struct_name))
    })?;

    // 6. 查找字段索引
    let field_names = ctx.struct_fields.get(struct_name).ok_or_else(|| {
        CodegenError::UnsupportedType(format!("Unknown struct '{}'", struct_name))
    })?;

    let index = field_names
        .iter()
        .position(|n| n == field_name)
        .ok_or_else(|| {
            CodegenError::UnsupportedType(format!(
                "Struct '{}' has no field '{}'",
                struct_name, field_name
            ))
        })?;

    // 7. 生成 GEP
    let field_ptr = ctx
        .builder
        .build_struct_gep(
            *struct_type,
            ptr_val,
            index as u32,
            &format!("field_{}_ptr", field_name),
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    Ok(field_ptr)
}

/// 内部辅助：从已有 CodegenValue 生成成员指针
pub fn gen_struct_member_ptr_val<'ctx>(
    ctx: &CodegenContext<'ctx>,
    object_val: &CodegenValue<'ctx>,
    _object_span_start: usize, // Needed for line info
    field_name: &str,
    _line: u32,
) -> CodegenResult<PointerValue<'ctx>> {
    // 2. 必须是指针类型（结构体是通过指针传递的）
    if !object_val.value.is_pointer_value() {
        return Err(CodegenError::UnsupportedType(
            "Field access on non-pointer value [Helper]".to_string(),
        ));
    }
    let ptr_val = object_val.value.into_pointer_value();

    // 运行时 Null 检查 (Caller might have done it, e.g. safe access. But if not optional, standard check)
    // Wait, this function shouldn't check null if it's called from safe access!
    // But gen_struct_member_ptr (unsafe) calls it.
    // So we should have a flag? Or separate logic.
    // "gen_struct_member_ptr_val_unchecked"?
    // Or just "gen_struct_member_ptr_val" and let caller handle check.
    // Standard access needs check. Safe access needs check (but branches).
    // So raw GEP generation should not check.

    // 3. 获取结构体名称和 LLVM 类型
    let struct_name = match &object_val.ty {
        Type::Struct(name) => name,
        Type::Nullable(inner) => match &**inner {
            Type::Struct(name) => name,
            _ => return Err(CodegenError::TypeMismatch),
        },
        _ => {
            return Err(CodegenError::UnsupportedType(format!(
                "Field access on non-struct type: {:?}",
                object_val.ty
            )))
        }
    };

    let struct_type = ctx.struct_types.get(struct_name).ok_or_else(|| {
        CodegenError::UnsupportedType(format!("Unknown struct '{}'", struct_name))
    })?;

    // 6. 查找字段索引
    let field_names = ctx.struct_fields.get(struct_name).ok_or_else(|| {
        CodegenError::UnsupportedType(format!("Unknown struct '{}'", struct_name))
    })?;

    let index = field_names
        .iter()
        .position(|n| n == field_name)
        .ok_or_else(|| {
            CodegenError::UnsupportedType(format!(
                "Struct '{}' has no field '{}'",
                struct_name, field_name
            ))
        })?;

    // 7. 生成 GEP
    let field_ptr = ctx
        .builder
        .build_struct_gep(
            *struct_type,
            ptr_val,
            index as u32,
            &format!("field_{}_ptr", field_name),
        )
        .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

    Ok(field_ptr)
}
