use super::common::load_field;
use super::ptr::gen_struct_member_ptr_val;
use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// 生成成员访问（RValue）
pub fn gen_member_access<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (inkwell::values::PointerValue<'ctx>, lency_syntax::ast::Type)>,
    object_expr: &Expr,
    field_name: &str,
    line: u32,
) -> CodegenResult<CodegenValue<'ctx>> {
    // 0. Check for Enum Static Access (Enum.Variant)
    if let lency_syntax::ast::ExprKind::Variable(name) = &object_expr.kind {
        if ctx.enum_types.contains(name) {
            // It is an Enum! Check if variant exists
            if let Some(variants) = ctx.enum_variants.get(name) {
                // variants is Vec<(VariantName, Fields)>
                if let Some((_, fields)) = variants.iter().find(|(vname, _)| vname == field_name) {
                    // Check if Unit or Tuple
                    if fields.is_empty() {
                        // Unit Variant: Generate Call to Enum_Variant()
                        let ctor_name = format!("{}_{}", name, field_name);
                        let function = ctx
                            .module
                            .get_function(&ctor_name)
                            .ok_or_else(|| CodegenError::FunctionNotFound(ctor_name.clone()))?;

                        let call_val = ctx
                            .builder
                            .build_call(function, &[], &format!("{}_call", ctor_name))
                            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
                        let basic_val = call_val.try_as_basic_value().left().ok_or(
                            CodegenError::LLVMBuildError("Constructor returned void".into()),
                        )?;

                        // Return Enum Type
                        // We need key for Type lookup.
                        // Enum is Opaque Struct.
                        // But we return Lency Type.
                        return Ok(CodegenValue {
                            value: basic_val,
                            ty: Type::Struct(name.clone()),
                        });
                    } else {
                        // Tuple Variant used as Getter?
                        // Option.Some -> Not a value in LLVM.
                        // Cannot generate code for it unless we support function pointers.
                        // But `infer_get` allows it.
                        // If we are here, Sema passed it.
                        // Maybe used in a context Codegen can't handle?
                        return Err(CodegenError::UnsupportedExpression);
                    }
                }
            }
        }
    }

    let object_val = generate_expr(ctx, locals, object_expr)?;

    // 特殊处理数组 length
    if let Type::Array { size, .. } = &object_val.ty {
        if field_name == "length" {
            return Ok(CodegenValue {
                value: ctx
                    .context
                    .i64_type()
                    .const_int((*size) as u64, false)
                    .into(),
                ty: Type::Int,
            });
        }
    }

    // Null check for standard access
    if object_val.value.is_pointer_value() {
        if let Some(panic_func) = ctx.panic_func {
            crate::runtime::gen_null_check(
                ctx.context,
                &ctx.builder,
                panic_func,
                object_val.value.into_pointer_value(),
                line,
            );
        }
    }

    // Check pointer or value
    if object_val.value.is_pointer_value() {
        let field_ptr =
            gen_struct_member_ptr_val(ctx, &object_val, object_expr.span.start, field_name, line)?;
        load_field(ctx, &object_val, field_name, field_ptr)
    } else {
        // Struct Value (RValue Aggregate) - use ExtractValue
        let struct_name = match &object_val.ty {
            Type::Struct(name) => name,
            _ => return Err(CodegenError::TypeMismatch),
        };

        let field_names = ctx
            .struct_fields
            .get(struct_name)
            .ok_or(CodegenError::TypeMismatch)?;
        let idx = field_names
            .iter()
            .position(|n| n == field_name)
            .ok_or(CodegenError::TypeMismatch)?;
        let field_types = ctx.struct_field_types.get(struct_name).unwrap();
        let ret_type = field_types[idx].clone();

        let val = ctx
            .builder
            .build_extract_value(
                object_val.value.into_struct_value(),
                idx as u32,
                &format!("field_{}_extract", field_name),
            )
            .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

        Ok(CodegenValue {
            value: val,
            ty: ret_type,
        })
    }
}
