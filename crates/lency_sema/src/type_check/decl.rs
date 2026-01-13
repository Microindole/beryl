use super::TypeChecker;
use crate::error::SemanticError;

use lency_syntax::ast::{Decl, Stmt, Type};

pub fn check_decl(checker: &mut TypeChecker, decl: &mut Decl) {
    match decl {
        Decl::Function {
            name,
            params,
            return_type,
            body,
            span,
            ..
        } => {
            check_function(checker, name, params, return_type, body, span);
        }

        Decl::ExternFunction { .. } => {
            // Nothing to check for extern declarations (types checked at parser/resolver level implicitly)
        }
        Decl::Struct { generic_params, .. } => {
            // Struct 字段类型在 resolver 阶段已验证
            // 但如果 Resolver 为泛型参数创建了作用域，我们需要跳过它以保持索引同步
            if !generic_params.is_empty() {
                checker.next_child_index += 1;
            }
        }
        Decl::Impl { methods, .. } => {
            // 递归检查每个方法
            for method in methods {
                check_decl(checker, method);
            }
        }

        Decl::Trait { generic_params, .. } => {
            // Skip the scope created by Resolver for generic params
            if !generic_params.is_empty() {
                checker.next_child_index += 1;
            }
        }
        Decl::Enum { generic_params, .. } => {
            if !generic_params.is_empty() {
                checker.next_child_index += 1;
            }
        }
        Decl::Var {
            span: _,
            name,
            ty,
            value,
        } => {
            // Global Scopes are already active?
            // TypeChecker runs in global scope by default.

            // Infer value type
            // Note: infer_type needs &mut Expr
            let value_ty = match checker.infer_type(value) {
                Ok(t) => t,
                Err(e) => {
                    checker.errors.push(e);
                    lency_syntax::ast::Type::Void // Placeholder
                }
            };

            // Check type annotation
            if let Some(annotation) = ty {
                if annotation != &value_ty {
                    // Simple check for now
                    checker.errors.push(SemanticError::TypeMismatch {
                        expected: annotation.to_string(),
                        found: value_ty.to_string(),
                        span: value.span.clone(),
                    });
                }
            } else {
                // Update symbol type if inferred
                if let Some(id) = checker.scopes.lookup_id(name) {
                    if let Some(crate::symbol::Symbol::Variable(sym)) =
                        checker.scopes.get_symbol_mut(id)
                    {
                        sym.ty = value_ty;
                    }
                }
            }
        }
        Decl::Import { .. } => {} // No-op
    }
}

pub fn check_function(
    checker: &mut TypeChecker,
    name: &str,
    _params: &[lency_syntax::ast::Param],
    return_type: &Type,
    body: &mut [Stmt],
    span: &std::ops::Range<usize>,
) {
    // 保存当前作用域
    let parent_scope = checker.scopes.current_scope();

    // 获取所有子作用域（按创建顺序）
    let children = checker.scopes.get_child_scopes(parent_scope);

    // 进入下一个函数作用域
    if let Some(&func_scope) = children.get(checker.next_child_index) {
        checker.scopes.set_current(func_scope);
        checker.next_child_index += 1;
    }

    // 保存并重置子索引（为函数体内的子作用域做准备）
    let prev_child_index = checker.next_child_index;
    checker.next_child_index = 0;

    // 设置当前函数返回类型
    let prev_return = checker.current_return_type.replace(return_type.clone());

    // 检查函数体中的每个语句
    for stmt in body.iter_mut() {
        checker.check_stmt(stmt);
    }

    // 检查非 void 函数是否有返回值
    if *return_type != Type::Void && !checker.has_return(body) {
        checker.errors.push(SemanticError::MissingReturn {
            name: name.to_string(),
            ty: return_type.to_string(),
            span: span.clone(),
        });
    }

    // 恢复返回类型
    checker.current_return_type = prev_return;

    // 恢复子索引
    checker.next_child_index = prev_child_index;

    // 恢复作用域
    checker.scopes.set_current(parent_scope);
}
