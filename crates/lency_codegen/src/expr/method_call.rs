//! Method Call Code Generation
//!
//! 处理方法调用：object.method(args)

use crate::context::CodegenContext;
use crate::error::{CodegenError, CodegenResult};
use crate::expr::{generate_expr, CodegenValue};
use inkwell::values::PointerValue;
use lency_syntax::ast::{Expr, Type};
use std::collections::HashMap;

/// 生成方法调用代码
pub fn gen_method_call<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (PointerValue<'ctx>, Type)>,
    object: &Expr,
    method_name: &str,
    args: &[Expr],
    line: u32,
) -> CodegenResult<CodegenValue<'ctx>> {
    // 0. Check for Enum Constructor Call (Enum.Variant(args))
    let enum_check = match &object.kind {
        lency_syntax::ast::ExprKind::Variable(name) => Some(name.clone()),
        lency_syntax::ast::ExprKind::GenericInstantiation { base, .. } => {
            if let lency_syntax::ast::ExprKind::Variable(name) = &base.kind {
                Some(name.clone())
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some(name) = enum_check {
        if ctx.enum_types.contains(&name) {
            // It's an Enum Constructor!
            let ctor_name = format!("{}_{}", name, method_name);
            let function = ctx
                .module
                .get_function(&ctor_name)
                .ok_or_else(|| CodegenError::FunctionNotFound(ctor_name.clone()))?;

            let mut compiled_args = Vec::with_capacity(args.len());
            for arg in args {
                let arg_val = generate_expr(ctx, locals, arg)?;
                compiled_args.push(arg_val.value.into());
            }

            let call_site = ctx
                .builder
                .build_call(function, &compiled_args, "call_ctor")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            let val = call_site
                .try_as_basic_value()
                .left()
                .ok_or(CodegenError::LLVMBuildError(
                    "Constructor returned void".into(),
                ))?;

            return Ok(CodegenValue {
                value: val,
                ty: Type::Struct(name.clone()),
            });
        }
    }

    // 1. 计算对象表达式
    let object_val = generate_expr(ctx, locals, object)?;

    // 2. 根据对象类型分发
    // Clone type to avoid borrowing object_val, so we can move it
    let object_type = object_val.ty.clone();

    match object_type {
        Type::Vec(inner) => crate::expr::vec::gen_vec_method_call(
            ctx,
            locals,
            object_val,
            method_name,
            args,
            &inner,
        ),
        // Primitive types: int, string, bool
        Type::Int | Type::String | Type::Bool => {
            // 获取类型名称用于 mangling
            let type_name = match &object_val.ty {
                Type::Int => "int",
                Type::String => "string",
                Type::Bool => "bool",
                _ => unreachable!(),
            };

            // 构建 mangled name: int_hash, string_eq 等
            let mangled_name = format!("{}_{}", type_name, method_name);

            // 查找函数
            let function = ctx
                .module
                .get_function(&mangled_name)
                .ok_or_else(|| CodegenError::FunctionNotFound(mangled_name.clone()))?;

            // 生成参数列表
            // 对于 primitive types，第一个参数是 this 的值（不是指针）
            let mut compiled_args = Vec::with_capacity(args.len() + 1);

            // 将 this 值作为第一个参数
            compiled_args.push(object_val.value.into());

            // 添加其他参数
            for arg in args {
                let arg_val = generate_expr(ctx, locals, arg)?;
                compiled_args.push(arg_val.value.into());
            }

            // 获取返回类型
            let return_type = ctx
                .function_signatures
                .get(&mangled_name)
                .cloned()
                .ok_or_else(|| CodegenError::FunctionNotFound(mangled_name.clone()))?;

            // 生成调用
            let call_site = ctx
                .builder
                .build_call(function, &compiled_args, "call_method")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // 处理返回值
            let val = call_site.try_as_basic_value().left();

            if let Some(v) = val {
                Ok(CodegenValue {
                    value: v,
                    ty: return_type,
                })
            } else {
                // Void 返回，生成 dummy 值
                let dummy = ctx.context.bool_type().const_int(0, false).into();
                Ok(CodegenValue {
                    value: dummy,
                    ty: Type::Void,
                })
            }
        }
        Type::Struct(name) => {
            // 获取 this 指针
            let this_ptr = if object_val.value.is_pointer_value() {
                object_val.value.into_pointer_value()
            } else {
                // 如果是值（右值结构体），分配临时空间
                let struct_type = object_val.value.get_type();
                let alloca = ctx
                    .builder
                    .build_alloca(struct_type, "this_tmp")
                    .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
                ctx.builder
                    .build_store(alloca, object_val.value)
                    .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;
                alloca
            };

            let struct_name = name.clone();

            // 运行时 Null 检查
            if let Some(panic_func) = ctx.panic_func {
                crate::runtime::gen_null_check(
                    ctx.context,
                    &ctx.builder,
                    panic_func,
                    this_ptr,
                    line,
                );
            }

            // 构建 mangled name
            let mangled_name = format!("{}_{}", struct_name, method_name);

            // 查找函数
            let function = ctx
                .module
                .get_function(&mangled_name)
                .ok_or_else(|| CodegenError::FunctionNotFound(mangled_name.clone()))?;

            // 生成参数列表
            let mut compiled_args = Vec::with_capacity(args.len() + 1);

            // 将 this_ptr 作为第一个参数
            compiled_args.push(this_ptr.into());

            // 添加其他参数
            for arg in args {
                let arg_val = generate_expr(ctx, locals, arg)?;
                compiled_args.push(arg_val.value.into());
            }

            // 获取返回类型
            let return_type = ctx
                .function_signatures
                .get(&mangled_name)
                .cloned()
                .ok_or_else(|| CodegenError::FunctionNotFound(mangled_name.clone()))?;

            // 生成调用
            let call_site = ctx
                .builder
                .build_call(function, &compiled_args, "call_method")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // 处理返回值
            let val = call_site.try_as_basic_value().left();

            if let Some(v) = val {
                Ok(CodegenValue {
                    value: v,
                    ty: return_type,
                })
            } else {
                // Void 返回，生成 dummy 值
                let dummy = ctx.context.bool_type().const_int(0, false).into();
                Ok(CodegenValue {
                    value: dummy,
                    ty: Type::Void,
                })
            }
        }
        Type::Result { ok_type, err_type } => {
            // Result 类型方法调用
            // Result 使用指针语义，类似 Struct
            let this_ptr = if object_val.value.is_pointer_value() {
                object_val.value.into_pointer_value()
            } else {
                return Err(CodegenError::TypeMismatch);
            };

            // 尝试使用内置方法实现
            if let Some(result) = gen_result_builtin_method(
                ctx,
                locals,
                this_ptr,
                method_name,
                args,
                &ok_type,
                &err_type,
            )? {
                return Ok(result);
            }

            // Fallback: 查找编译的方法函数
            // 构建 mangled 方法名：Result__int_Error_unwrap_or
            let result_type_mangled = lency_monomorph::mangling::mangle_type(&Type::Result {
                ok_type: ok_type.clone(),
                err_type: err_type.clone(),
            });
            let mangled_name = format!("{}_{}", result_type_mangled, method_name);

            // 查找函数
            let function = ctx
                .module
                .get_function(&mangled_name)
                .ok_or_else(|| CodegenError::FunctionNotFound(mangled_name.clone()))?;

            // 生成参数列表
            let mut compiled_args = Vec::with_capacity(args.len() + 1);

            // 将 this_ptr 作为第一个参数
            compiled_args.push(this_ptr.into());

            // 添加其他参数
            for arg in args {
                let arg_val = generate_expr(ctx, locals, arg)?;
                compiled_args.push(arg_val.value.into());
            }

            // 获取返回类型
            let return_type = ctx
                .function_signatures
                .get(&mangled_name)
                .cloned()
                .ok_or_else(|| CodegenError::FunctionNotFound(mangled_name.clone()))?;

            // 生成调用
            let call_site = ctx
                .builder
                .build_call(function, &compiled_args, "call_result_method")
                .map_err(|e| CodegenError::LLVMBuildError(e.to_string()))?;

            // 处理返回值
            let val = call_site.try_as_basic_value().left();

            if let Some(v) = val {
                Ok(CodegenValue {
                    value: v,
                    ty: return_type,
                })
            } else {
                // Void 返回，生成 dummy 值
                let dummy = ctx.context.bool_type().const_int(0, false).into();
                Ok(CodegenValue {
                    value: dummy,
                    ty: Type::Void,
                })
            }
        }
        _ => Err(CodegenError::TypeMismatch),
    }
}

// 移除 gen_method_call_fallback，不再需要
#[allow(dead_code)]
fn _unused() {}

/// Result 内置方法实现
///
/// 直接读取 Result 结构的内部字段，无需 match 语法
/// Result 内存布局:
///   index 0: is_ok (bool)
///   index 1: ok_value (T)  [如果 T != void]
///   index 2: err_value (E) [或 index 1 如果 T == void]
fn gen_result_builtin_method<'ctx>(
    ctx: &CodegenContext<'ctx>,
    locals: &HashMap<String, (PointerValue<'ctx>, Type)>,
    result_ptr: PointerValue<'ctx>,
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
