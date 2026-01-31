//! Option Intrinsic Methods
//!
//! 内置实现 Option<T> 的方法：is_some, is_none, unwrap_or

use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use crate::types::ToLLVMType;
use inkwell::values::PointerValue;
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// Option 内置方法实现
///
/// 直接读取 Option 结构的内部字段
/// Option 内存布局 (通用 Enum):
///   index 0: tag (i64)  [0 = Some, 1 = None]
///   index 1: payload ([size x i8])
pub fn gen_option_builtin_method<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (PointerValue<'ctx>, Type)>,
    option_ptr: PointerValue<'ctx>,
    method_name: &str,
    args: &[Expr],
    struct_name: &str,     // 例如 "Option_int"
    generic_args: &[Type], // T
) -> CodegenResult<Option<CodegenValue<'ctx>>> {
    let struct_type = match ctx.struct_types.get(struct_name) {
        Some(st) => *st,
        None => return Ok(None),
    };

    match method_name {
        "is_some" => {
            // tag == 0 ?
            let tag_ptr = ctx
                .builder
                .build_struct_gep(struct_type, option_ptr, 0, "tag_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let tag_val = ctx
                .builder
                .build_load(ctx.context.i64_type(), tag_ptr, "tag_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_int_value();

            let is_some = ctx
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    tag_val,
                    ctx.context.i64_type().const_int(0, false),
                    "is_some",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            Ok(Some(CodegenValue {
                value: is_some.into(),
                ty: Type::Bool,
            }))
        }
        "is_none" => {
            // tag == 1 ?
            let tag_ptr = ctx
                .builder
                .build_struct_gep(struct_type, option_ptr, 0, "tag_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let tag_val = ctx
                .builder
                .build_load(ctx.context.i64_type(), tag_ptr, "tag_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_int_value();

            let is_none = ctx
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    tag_val,
                    ctx.context.i64_type().const_int(1, false),
                    "is_none",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            Ok(Some(CodegenValue {
                value: is_none.into(),
                ty: Type::Bool,
            }))
        }
        "unwrap_or" => {
            if args.len() != 1 {
                return Ok(None);
            }

            // 1. 计算默认值
            let default_val = generate_expr(ctx, locals, &args[0])?;
            // 推断 T 类型，从 default_val 的类型推断最准确
            let t_type = default_val.ty.clone();

            // 2. 检查 Tag (0 = Some)
            let tag_ptr = ctx
                .builder
                .build_struct_gep(struct_type, option_ptr, 0, "tag_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let tag_val = ctx
                .builder
                .build_load(ctx.context.i64_type(), tag_ptr, "tag_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_int_value();

            let is_some = ctx
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    tag_val,
                    ctx.context.i64_type().const_int(0, false),
                    "is_some",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // 3. 加载 Some(T) 的值
            // Option 内存布局: struct { i64 tag, [size x i8] payload }
            // 我们需要把 payload 指针 bitcast 成 { T }* 然后取第 0 个元素

            let payload_arr_ptr = ctx
                .builder
                .build_struct_gep(struct_type, option_ptr, 1, "payload_arr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // Create variant struct { T } type
            // use crate::types::ToLLVMType;
            let t_llvm_type = t_type.to_llvm_type(ctx)?;
            let variant_struct_type = ctx.context.struct_type(&[t_llvm_type], false);

            let payload_typed_ptr = ctx
                .builder
                .build_bitcast(
                    payload_arr_ptr,
                    variant_struct_type.ptr_type(inkwell::AddressSpace::default()),
                    "payload_typed",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_pointer_value();

            // GEP to get T*
            let val_ptr = ctx
                .builder
                .build_struct_gep(variant_struct_type, payload_typed_ptr, 0, "some_val_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            let some_val = ctx
                .builder
                .build_load(t_llvm_type, val_ptr, "some_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // 4. Select
            let result = ctx
                .builder
                .build_select(is_some, some_val, default_val.value, "unwrap_or_res")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            Ok(Some(CodegenValue {
                value: result,
                ty: t_type,
            }))
        }
        "unwrap" => {
            if !args.is_empty() {
                return Ok(None);
            }

            // check tag == 0 (Some)
            let tag_ptr = ctx
                .builder
                .build_struct_gep(struct_type, option_ptr, 0, "tag_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            let tag_val = ctx
                .builder
                .build_load(ctx.context.i64_type(), tag_ptr, "tag_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_int_value();

            let is_some = ctx
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    tag_val,
                    ctx.context.i64_type().const_int(0, false),
                    "is_some",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // Block setup for panic
            let func = ctx
                .builder
                .get_insert_block()
                .unwrap()
                .get_parent()
                .unwrap();

            let then_bb = ctx.context.append_basic_block(func, "unwrap_some");
            let else_bb = ctx.context.append_basic_block(func, "unwrap_none");
            let merge_bb = ctx.context.append_basic_block(func, "unwrap_merge");

            ctx.builder
                .build_conditional_branch(is_some, then_bb, else_bb)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // Emit Some block
            ctx.builder.position_at_end(then_bb);

            // Extract T (needed for return type)
            let payload_arr_ptr = ctx
                .builder
                .build_struct_gep(struct_type, option_ptr, 1, "payload_arr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // Retrieve T from generic_args (added to signature)
            let t_type = if let Some(ty) = generic_args.first() {
                ty.clone()
            } else {
                // Fallback: Parse T from struct_name "Option__int", "Option__MyStruct"
                if let Some(type_name) = struct_name.strip_prefix("Option__") {
                    match type_name {
                        "int" => Type::Int,
                        "bool" => Type::Bool,
                        "string" => Type::String,
                        "float" => Type::Float,
                        _ => Type::Struct(type_name.to_string()),
                    }
                } else {
                    return Err(CodegenError::LLVMBuildError(
                        "Option missing generic arg T".to_string(),
                    ));
                }
            };

            let t_llvm_type = t_type.to_llvm_type(ctx)?;
            let variant_struct_type = ctx.context.struct_type(&[t_llvm_type], false);

            let payload_typed_ptr = ctx
                .builder
                .build_bitcast(
                    payload_arr_ptr,
                    variant_struct_type.ptr_type(inkwell::AddressSpace::default()),
                    "payload_typed",
                )
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?
                .into_pointer_value();

            // GEP to get T*
            let val_ptr = ctx
                .builder
                .build_struct_gep(variant_struct_type, payload_typed_ptr, 0, "some_val_ptr")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            let some_val = ctx
                .builder
                .build_load(t_llvm_type, val_ptr, "some_val")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            ctx.builder
                .build_unconditional_branch(merge_bb)
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // Emit None block (Panic!)
            ctx.builder.position_at_end(else_bb);
            if let Some(panic_func) = ctx.panic_func {
                crate::runtime::gen_panic(
                    ctx.context,
                    &ctx.builder,
                    panic_func,
                    "Option::unwrap called on None",
                    0,
                );
            } else {
                ctx.builder.build_unreachable().unwrap();
            }

            // Merge block
            ctx.builder.position_at_end(merge_bb);
            let phi = ctx
                .builder
                .build_phi(t_llvm_type, "unwrap_res")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
            phi.add_incoming(&[(&some_val, then_bb)]);

            Ok(Some(CodegenValue {
                value: phi.as_basic_value(),
                ty: t_type,
            }))
        }
        _ => Ok(None),
    }
}
