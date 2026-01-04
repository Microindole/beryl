use crate::resolver::Resolver;
use crate::scope::ScopeKind;
use crate::symbol::{FunctionSymbol, ParameterSymbol};
use crate::{SemanticError, Symbol};
use beryl_syntax::ast::{Decl, Type};

/// 收集顶层声明（Pass 1）
pub fn collect_decl(resolver: &mut Resolver, decl: &Decl) {
    match decl {
        Decl::Function {
            name,
            params,
            return_type,
            span,
            ..
        } => {
            let func_symbol = FunctionSymbol::new(
                name.clone(),
                params
                    .iter()
                    .map(|p| (p.name.clone(), p.ty.clone()))
                    .collect(),
                return_type.clone(),
                span.clone(),
            );

            if let Err(e) = resolver.scopes.define(Symbol::Function(func_symbol)) {
                resolver.errors.push(e);
            }
        }

        Decl::ExternFunction {
            name,
            params,
            return_type,
            span,
            ..
        } => {
            let func_symbol = FunctionSymbol::new(
                name.clone(),
                params
                    .iter()
                    .map(|p| (p.name.clone(), p.ty.clone()))
                    .collect(),
                return_type.clone(),
                span.clone(),
            );

            if let Err(e) = resolver.scopes.define(Symbol::Function(func_symbol)) {
                resolver.errors.push(e);
            }
        }
        Decl::Struct {
            name, fields, span, ..
        } => {
            let mut struct_symbol = crate::symbol::StructSymbol::new(name.clone(), span.clone());

            // 收集字段
            for field in fields {
                struct_symbol.add_field(field.name.clone(), field.ty.clone(), span.clone());
            }

            if let Err(e) = resolver.scopes.define(Symbol::Struct(struct_symbol)) {
                resolver.errors.push(e);
            }
        }
        Decl::Impl {
            type_name,
            methods,
            span,
            ..
        } => {
            // 查找对应的 Struct
            let struct_id = resolver.scopes.lookup_id(type_name);
            if struct_id.is_none() {
                resolver.errors.push(SemanticError::UndefinedType {
                    name: type_name.clone(),
                    span: span.clone(),
                });
                return;
            }

            // 获取 StructSymbol 的可变引用
            let struct_id = struct_id.unwrap();
            if let Some(Symbol::Struct(struct_sym)) = resolver.scopes.get_symbol_mut(struct_id) {
                // 为每个方法创建 FunctionSymbol 并注册
                for method in methods {
                    if let Decl::Function {
                        name,
                        params,
                        return_type,
                        span,
                        ..
                    } = method
                    {
                        let func_symbol = FunctionSymbol::new(
                            name.clone(),
                            params
                                .iter()
                                .map(|p| (p.name.clone(), p.ty.clone()))
                                .collect(),
                            return_type.clone(),
                            span.clone(),
                        );
                        struct_sym.add_method(name.clone(), func_symbol);
                    }
                }
            } else {
                resolver.errors.push(SemanticError::NotAStruct {
                    name: type_name.clone(),
                    span: span.clone(),
                });
            }
        }
    }
}

/// 解析声明（Pass 2）
pub fn resolve_decl(resolver: &mut Resolver, decl: &Decl) {
    match decl {
        Decl::Function {
            name: _,
            params,
            body,
            span,
            ..
        } => {
            // 进入函数作用域
            resolver.scopes.enter_scope(ScopeKind::Function);

            // 注册参数
            for (i, param) in params.iter().enumerate() {
                let param_symbol =
                    ParameterSymbol::new(param.name.clone(), param.ty.clone(), span.clone(), i);
                if let Err(e) = resolver.scopes.define(Symbol::Parameter(param_symbol)) {
                    resolver.errors.push(e);
                }
            }

            // 解析函数体
            for stmt in body {
                resolver.resolve_stmt(stmt);
            }

            // 退出函数作用域
            resolver.scopes.exit_scope();
        }

        Decl::ExternFunction { .. } => {
            // No body to resolve
        }
        Decl::Struct {
            name: _,
            fields,
            span,
            ..
        } => {
            // 验证字段类型是否存在（特别是自定义 Struct 类型）
            for field in fields {
                if let Type::Struct(type_name) = &field.ty {
                    // 检查引用的 Struct 类型是否已定义
                    if resolver.scopes.lookup(type_name).is_none() {
                        resolver.errors.push(SemanticError::UndefinedType {
                            name: type_name.clone(),
                            span: span.clone(),
                        });
                    }
                }
                // 其他基本类型（int, string 等）不需要验证
            }
        }
        Decl::Impl {
            type_name,
            methods,
            span,
            ..
        } => {
            // 验证 Struct 存在
            let struct_id = resolver.scopes.lookup_id(type_name);
            if struct_id.is_none() {
                resolver.errors.push(SemanticError::UndefinedType {
                    name: type_name.clone(),
                    span: span.clone(),
                });
                return;
            }

            let struct_id = struct_id.unwrap();
            if !matches!(
                resolver.scopes.get_symbol(struct_id),
                Some(Symbol::Struct(_))
            ) {
                resolver.errors.push(SemanticError::NotAStruct {
                    name: type_name.clone(),
                    span: span.clone(),
                });
                return;
            }

            // 解析每个方法（添加隐式 this 参数）
            for method in methods {
                if let Decl::Function {
                    params, body, span, ..
                } = method
                {
                    // 进入方法作用域
                    resolver.scopes.enter_scope(ScopeKind::Function);

                    // 注册隐式 this 参数
                    let this_type = Type::Struct(type_name.clone());
                    let this_param = ParameterSymbol::new(
                        "this".to_string(),
                        this_type,
                        span.clone(),
                        0, // this 是第一个参数
                    );
                    if let Err(e) = resolver.scopes.define(Symbol::Parameter(this_param)) {
                        resolver.errors.push(e);
                    }

                    // 注册其他参数（索引从 1 开始）
                    for (i, param) in params.iter().enumerate() {
                        let param_symbol = ParameterSymbol::new(
                            param.name.clone(),
                            param.ty.clone(),
                            span.clone(),
                            i + 1, // this 是 0，所以从 1 开始
                        );
                        if let Err(e) = resolver.scopes.define(Symbol::Parameter(param_symbol)) {
                            resolver.errors.push(e);
                        }
                    }

                    // 解析方法体
                    for stmt in body {
                        resolver.resolve_stmt(stmt);
                    }

                    // 退出方法作用域
                    resolver.scopes.exit_scope();
                }
            }
        }
    }
}
