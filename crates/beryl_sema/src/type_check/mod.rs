//! Type Checking
//!
//! 类型检查模块，验证程序的类型正确性。
//! 遵循 Beryl "Safety by Default" 哲学：严格的类型检查，拒绝隐式错误。

use crate::error::SemanticError;
use crate::scope::ScopeStack;
use crate::symbol::{FunctionSymbol, Symbol};
use crate::type_infer::{is_compatible, TypeInferer};
use beryl_syntax::ast::{Decl, Expr, ExprKind, Program, Stmt, Type};
use std::collections::HashMap;

pub mod decl;
pub mod stmt;

/// 类型检查器
pub struct TypeChecker<'a> {
    pub(crate) scopes: &'a mut ScopeStack,
    pub(crate) errors: Vec<SemanticError>,
    /// 当前函数的返回类型（用于检查 return 语句）
    pub(crate) current_return_type: Option<Type>,
    /// 下一个要处理的子作用域索引（用于同步作用域遍历）
    pub(crate) next_child_index: usize,
    /// 当前循环嵌套深度
    pub(crate) loop_depth: usize,
}

impl<'a> TypeChecker<'a> {
    pub fn new(scopes: &'a mut ScopeStack) -> Self {
        Self {
            scopes,
            errors: Vec::new(),
            current_return_type: None,
            next_child_index: 0,
            loop_depth: 0,
        }
    }

    /// 检查整个程序
    pub fn check(&mut self, program: &mut Program) -> Result<(), Vec<SemanticError>> {
        for decl in &mut program.decls {
            self.check_decl(decl);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// 检查声明 (delegate to module)
    pub fn check_decl(&mut self, decl: &mut Decl) {
        decl::check_decl(self, decl);
    }

    /// 检查语句 (delegate to module)
    pub fn check_stmt(&mut self, stmt: &mut Stmt) {
        stmt::check_stmt(self, stmt);
    }

    /// 替换类型中的泛型参数
    fn substitute_type(&self, ty: &Type, mapping: &HashMap<String, Type>) -> Type {
        match ty {
            Type::GenericParam(name) => {
                if let Some(concrete) = mapping.get(name) {
                    concrete.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Generic(name, args) => {
                let new_args = args
                    .iter()
                    .map(|arg| self.substitute_type(arg, mapping))
                    .collect();
                Type::Generic(name.clone(), new_args)
            }
            Type::Vec(inner) => Type::Vec(Box::new(self.substitute_type(inner, mapping))),
            Type::Array { element_type, size } => Type::Array {
                element_type: Box::new(self.substitute_type(element_type, mapping)),
                size: *size,
            },
            Type::Nullable(inner) => Type::Nullable(Box::new(self.substitute_type(inner, mapping))),
            Type::Struct(name) => {
                if let Some(concrete) = mapping.get(name) {
                    concrete.clone()
                } else {
                    ty.clone()
                }
            }
            _ => ty.clone(),
        }
    }

    /// 检查函数调用
    pub fn check_call(
        &mut self,
        callee: &mut Expr,
        args: &mut [Expr],
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        // 解析被调用者 (包括泛型实例化)
        let (func, is_method, subst_map) = match &mut callee.kind {
            ExprKind::GenericInstantiation {
                base,
                args: type_args,
            } => {
                // 泛型函数调用: func::<T>(...)
                match &mut base.kind {
                    ExprKind::Variable(name) => {
                        match self.scopes.lookup(name) {
                            Some(Symbol::Function(f)) => {
                                // Check generic arg count
                                if f.generic_params.len() != type_args.len() {
                                    return Err(SemanticError::ArgumentCountMismatch {
                                        name: format!("{} generic args", name),
                                        expected: f.generic_params.len(),
                                        found: type_args.len(),
                                        span: span.clone(),
                                    });
                                }

                                // Build substitution map
                                let mut map = HashMap::new();
                                for (param, arg_ty) in f.generic_params.iter().zip(type_args.iter())
                                {
                                    map.insert(param.name.as_str().to_string(), arg_ty.clone());
                                }
                                (f.clone(), false, map)
                            }
                            _ => {
                                return Err(SemanticError::NotCallable {
                                    ty: name.clone(),
                                    span: span.clone(),
                                })
                            }
                        }
                    }
                    _ => {
                        return Err(SemanticError::NotCallable {
                            ty: "complex generic expression".to_string(),
                            span: span.clone(),
                        })
                    }
                }
            }
            ExprKind::Variable(name) => {
                // 普通函数调用
                match self.scopes.lookup(name) {
                    Some(Symbol::Function(f)) => (f.clone(), false, HashMap::new()),
                    Some(Symbol::Struct(s)) => {
                        // 构造函数
                        let func_sym = FunctionSymbol {
                            name: name.clone(),
                            params: s
                                .fields
                                .iter()
                                .map(|(fname, finfo)| (fname.clone(), finfo.ty.clone()))
                                .collect(),
                            return_type: Type::Struct(name.clone()),
                            generic_params: s.generic_params.clone(), // Struct generic params
                            span: s.span.clone(),
                            is_public: true, // Constructors are usually public or match struct visibility
                        };
                        (func_sym, false, HashMap::new())
                    }
                    _ => {
                        return Err(SemanticError::NotCallable {
                            ty: name.clone(),
                            span: span.clone(),
                        });
                    }
                }
            }
            ExprKind::Get { object, name } => {
                // 方法调用处理
                let obj_type = self.infer_type(object)?;
                match obj_type {
                    Type::Struct(struct_name) => {
                        // 构建 mangled name: StructName_methodName
                        let mangled_name = format!("{}_{}", struct_name, name);
                        match self.scopes.lookup(&mangled_name) {
                            Some(Symbol::Function(f)) => (f.clone(), true, HashMap::new()),
                            _ => {
                                return Err(SemanticError::UndefinedMethod {
                                    class: struct_name,
                                    method: name.clone(),
                                    span: span.clone(),
                                });
                            }
                        }
                    }
                    Type::Vec(inner_type) => {
                        // Vec 内置方法处理
                        match name.as_str() {
                            "push" => {
                                // push(val)
                                if args.len() != 1 {
                                    return Err(SemanticError::ArgumentCountMismatch {
                                        name: "push".to_string(),
                                        expected: 1,
                                        found: args.len(),
                                        span: span.clone(),
                                    });
                                }
                                let arg_ty = self.infer_type(&mut args[0])?;
                                if !is_compatible(&inner_type, &arg_ty) {
                                    return Err(SemanticError::TypeMismatch {
                                        expected: inner_type.to_string(),
                                        found: arg_ty.to_string(),
                                        span: args[0].span.clone(),
                                    });
                                }
                                return Ok(Type::Void);
                            }
                            "pop" => {
                                // pop() -> T
                                if !args.is_empty() {
                                    return Err(SemanticError::ArgumentCountMismatch {
                                        name: "pop".to_string(),
                                        expected: 0,
                                        found: args.len(),
                                        span: span.clone(),
                                    });
                                }
                                return Ok(*inner_type);
                            }
                            "len" => {
                                // len() -> int
                                if !args.is_empty() {
                                    return Err(SemanticError::ArgumentCountMismatch {
                                        name: "len".to_string(),
                                        expected: 0,
                                        found: args.len(),
                                        span: span.clone(),
                                    });
                                }
                                return Ok(Type::Int);
                            }
                            "get" => {
                                // get(index) -> T
                                if args.len() != 1 {
                                    return Err(SemanticError::ArgumentCountMismatch {
                                        name: "get".to_string(),
                                        expected: 1,
                                        found: args.len(),
                                        span: span.clone(),
                                    });
                                }
                                let arg_ty = self.infer_type(&mut args[0])?;
                                if !is_compatible(&Type::Int, &arg_ty) {
                                    return Err(SemanticError::TypeMismatch {
                                        expected: "int".to_string(),
                                        found: arg_ty.to_string(),
                                        span: args[0].span.clone(),
                                    });
                                }
                                return Ok(*inner_type);
                            }
                            "set" => {
                                // set(index, val) -> void
                                if args.len() != 2 {
                                    return Err(SemanticError::ArgumentCountMismatch {
                                        name: "set".to_string(),
                                        expected: 2,
                                        found: args.len(),
                                        span: span.clone(),
                                    });
                                }
                                let index_ty = self.infer_type(&mut args[0])?;
                                if !is_compatible(&Type::Int, &index_ty) {
                                    return Err(SemanticError::TypeMismatch {
                                        expected: "int".to_string(),
                                        found: index_ty.to_string(),
                                        span: args[0].span.clone(),
                                    });
                                }
                                let val_ty = self.infer_type(&mut args[1])?;
                                if !is_compatible(&inner_type, &val_ty) {
                                    return Err(SemanticError::TypeMismatch {
                                        expected: inner_type.to_string(),
                                        found: val_ty.to_string(),
                                        span: args[1].span.clone(),
                                    });
                                }
                                return Ok(Type::Void);
                            }
                            _ => {
                                return Err(SemanticError::UndefinedMethod {
                                    class: format!("Vec<{}>", inner_type),
                                    method: name.clone(),
                                    span: span.clone(),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(SemanticError::NotAStruct {
                            name: obj_type.to_string(),
                            span: object.span.clone(),
                        });
                    }
                }
            }
            _ => {
                // 复杂调用表达式暂不支持
                return Ok(Type::Error);
            }
        };

        // 检查参数数量
        // 如果是方法调用，定义中有隐式 this 参数，所以 args.len() + 1 应该等于 params.len()
        let expected_args = if is_method {
            func.params.len() - 1
        } else {
            func.params.len()
        };

        if args.len() != expected_args {
            return Err(SemanticError::ArgumentCountMismatch {
                name: func.name.clone(),
                expected: expected_args,
                found: args.len(),
                span: span.clone(),
            });
        }

        // 检查每个参数类型
        let skip_count = if is_method { 1 } else { 0 };
        let params_iter = func.params.iter().skip(skip_count);

        for (arg, (_, param_ty)) in args.iter_mut().zip(params_iter) {
            let arg_ty = self.infer_type(arg)?;
            // 关键：检查参数前先替换其中的泛型参数
            let expected_ty = self.substitute_type(param_ty, &subst_map);

            if !is_compatible(&expected_ty, &arg_ty) {
                self.errors.push(SemanticError::TypeMismatch {
                    expected: expected_ty.to_string(),
                    found: arg_ty.to_string(),
                    span: arg.span.clone(), // Use arg.span
                });
            }
        }

        // 返回类型的泛型替换
        Ok(self.substitute_type(&func.return_type, &subst_map))
    }

    /// 推导表达式类型（封装 TypeInferer）
    pub(crate) fn infer_type(&self, expr: &mut Expr) -> Result<Type, SemanticError> {
        let inferer = TypeInferer::new(self.scopes);
        inferer.infer(expr)
    }

    /// 检查代码块是否有返回语句
    pub(crate) fn has_return(&self, stmts: &[Stmt]) -> bool {
        for stmt in stmts {
            match stmt {
                Stmt::Return { .. } => return true,
                Stmt::If {
                    then_block,
                    else_block,
                    ..
                } => {
                    // 只有两个分支都有 return 才算完整覆盖
                    if self.has_return(then_block) {
                        if let Some(else_stmts) = else_block {
                            if self.has_return(else_stmts) {
                                return true;
                            }
                        }
                    }
                }
                Stmt::Block(inner) => {
                    if self.has_return(inner) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}
