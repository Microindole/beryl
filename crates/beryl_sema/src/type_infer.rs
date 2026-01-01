//! Type Inference
//!
//! 类型推导模块，处理 Beryl 中 `var x = 10` 这种省略类型声明的情况。
//! 遵循 "Crystal Clear" 哲学：推导规则透明可预测。

use crate::error::SemanticError;
use crate::operators::{BinaryOpRegistry, UnaryOpRegistry};
use crate::scope::{ScopeId, ScopeStack};
use crate::symbol::Symbol;
use beryl_syntax::ast::{Expr, ExprKind, Literal, Type, UnaryOp};

/// 类型推导器
///
/// 注意：TypeInferer 需要知道当前所在的作用域，
/// 因为变量查找需要从正确的作用域开始。
pub struct TypeInferer<'a> {
    scopes: &'a ScopeStack,
    /// 当前作用域 ID（由调用者设置，用于正确的符号查找）
    current_scope: ScopeId,
    /// 二元运算符注册表
    binary_ops: BinaryOpRegistry,
    /// 一元运算符注册表
    unary_ops: UnaryOpRegistry,
}

impl<'a> TypeInferer<'a> {
    pub fn new(scopes: &'a ScopeStack) -> Self {
        Self {
            scopes,
            current_scope: scopes.current_scope(),
            binary_ops: BinaryOpRegistry::new(),
            unary_ops: UnaryOpRegistry::new(),
        }
    }

    /// 创建一个指定作用域的推导器
    pub fn with_scope(scopes: &'a ScopeStack, scope_id: ScopeId) -> Self {
        Self {
            scopes,
            current_scope: scope_id,
            binary_ops: BinaryOpRegistry::new(),
            unary_ops: UnaryOpRegistry::new(),
        }
    }

    /// 设置当前作用域
    pub fn set_scope(&mut self, scope_id: ScopeId) {
        self.current_scope = scope_id;
    }

    /// 查找符号（从当前作用域向上）
    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.scopes.lookup_from(name, self.current_scope)
    }

    /// 推导表达式的类型
    pub fn infer(&self, expr: &Expr) -> Result<Type, SemanticError> {
        match &expr.kind {
            ExprKind::Literal(lit) => Ok(self.infer_literal(lit)),

            ExprKind::Variable(name) => self.infer_variable(name, &expr.span),

            ExprKind::Binary(left, op, right) => self.infer_binary(left, op, right, &expr.span),

            ExprKind::Unary(op, operand) => self.infer_unary(op, operand, &expr.span),

            ExprKind::Call { callee, args: _ } => self.infer_call(callee, &expr.span),

            ExprKind::Get { object, name } => self.infer_get(object, name, &expr.span),

            ExprKind::New {
                class_name,
                generics,
                args: _,
            } => self.infer_new(class_name, generics, &expr.span),

            ExprKind::Array(elements) => self.infer_array(elements, &expr.span),
            ExprKind::Match {
                value,
                cases,
                default,
            } => self.infer_match(value, cases, default.as_deref(), &expr.span),
            ExprKind::Print(expr) => {
                self.infer(expr)?;
                Ok(Type::Void)
            }
            ExprKind::Index { array, index } => self.infer_index(array, index, &expr.span),
        }
    }

    fn infer_match(
        &self,
        value: &Expr,
        cases: &[beryl_syntax::ast::MatchCase],
        default: Option<&Expr>,
        _span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        let value_ty = self.infer(value)?;
        if value_ty != Type::Int {
            return Err(SemanticError::TypeMismatch {
                expected: "int".to_string(),
                found: value_ty.to_string(),
                span: value.span.clone(),
            });
        }

        let mut ret_ty = Type::Error;
        let mut first = true;

        for case in cases {
            // Check pattern type (only Int literals supported for now)
            match &case.pattern {
                beryl_syntax::ast::MatchPattern::Literal(lit) => {
                    let pat_ty = self.infer_literal(lit);
                    if pat_ty != Type::Int {
                        return Err(SemanticError::TypeMismatch {
                            expected: "int".to_string(),
                            found: pat_ty.to_string(),
                            span: case.span.clone(),
                        });
                    }
                }
            }

            let body_ty = self.infer(&case.body)?;
            if first {
                ret_ty = body_ty;
                first = false;
            } else if !is_compatible(&ret_ty, &body_ty) {
                return Err(SemanticError::TypeMismatch {
                    expected: ret_ty.to_string(),
                    found: body_ty.to_string(),
                    span: case.body.span.clone(),
                });
            }
        }

        if let Some(def) = default {
            let def_ty = self.infer(def)?;
            if first {
                ret_ty = def_ty;
            } else if !is_compatible(&ret_ty, &def_ty) {
                return Err(SemanticError::TypeMismatch {
                    expected: ret_ty.to_string(),
                    found: def_ty.to_string(),
                    span: def.span.clone(),
                });
            }
        }

        Ok(ret_ty)
    }

    /// 推导字面量类型
    fn infer_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::String(_) => Type::String,
            Literal::Null => Type::Nullable(Box::new(Type::Error)), // null 需要上下文推导
        }
    }

    /// 推导变量类型
    fn infer_variable(
        &self,
        name: &str,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        match self.lookup(name) {
            Some(symbol) => {
                match symbol.ty() {
                    Some(ty) => Ok(ty.clone()),
                    None => {
                        // 函数名不是值类型
                        if let Symbol::Function(func) = symbol {
                            // 返回函数类型的占位（暂时用 Void 表示）
                            // 未来可扩展为 FunctionType
                            Ok(func.return_type.clone())
                        } else {
                            Ok(Type::Error)
                        }
                    }
                }
            }
            None => Err(SemanticError::UndefinedVariable {
                name: name.to_string(),
                span: span.clone(),
            }),
        }
    }

    /// 推导二元表达式类型
    ///
    /// 使用运算符注册表进行类型查找
    fn infer_binary(
        &self,
        left: &Expr,
        op: &beryl_syntax::ast::BinaryOp,
        right: &Expr,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        let left_ty = self.infer(left)?;
        let right_ty = self.infer(right)?;

        // 使用运算符表查找
        self.binary_ops.lookup(op, &left_ty, &right_ty, span)
    }

    /// 推导一元表达式类型
    ///
    /// 使用运算符注册表进行类型查找
    fn infer_unary(
        &self,
        op: &UnaryOp,
        operand: &Expr,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        let operand_ty = self.infer(operand)?;

        // 使用运算符表查找
        self.unary_ops.lookup(op, &operand_ty, span)
    }

    /// 推导函数调用类型
    fn infer_call(
        &self,
        callee: &Expr,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        // 获取被调用者的名称
        if let ExprKind::Variable(name) = &callee.kind {
            match self.lookup(name) {
                Some(Symbol::Function(func)) => Ok(func.return_type.clone()),
                Some(_) => Err(SemanticError::NotCallable {
                    ty: name.clone(),
                    span: span.clone(),
                }),
                None => Err(SemanticError::UndefinedFunction {
                    name: name.clone(),
                    span: span.clone(),
                }),
            }
        } else {
            // 复杂调用表达式（如 obj.method()），暂时返回 Error
            Ok(Type::Error)
        }
    }

    /// 推导成员访问类型
    fn infer_get(
        &self,
        object: &Expr,
        field_name: &str,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        let object_ty = self.infer(object)?;

        match &object_ty {
            Type::Class(class_name) => {
                // 查找类定义（类始终在全局作用域）
                match self.scopes.lookup_global(class_name) {
                    Some(Symbol::Class(class)) => {
                        // 查找字段
                        if let Some(field) = class.get_field(field_name) {
                            Ok(field.ty.clone())
                        } else {
                            Err(SemanticError::UndefinedField {
                                class: class_name.clone(),
                                field: field_name.to_string(),
                                span: span.clone(),
                            })
                        }
                    }
                    _ => Err(SemanticError::UndefinedType {
                        name: class_name.clone(),
                        span: span.clone(),
                    }),
                }
            }
            Type::Array { .. } => {
                if field_name == "length" {
                    Ok(Type::Int)
                } else {
                    Err(SemanticError::UndefinedField {
                        class: "Array".to_string(), // Array is not technicaly a class, but error msg fits
                        field: field_name.to_string(),
                        span: span.clone(),
                    })
                }
            }
            Type::Nullable(inner) => {
                // 可空类型需要先检查 null
                Err(SemanticError::PossibleNullAccess {
                    ty: format!("{}?", inner),
                    span: span.clone(),
                })
            }
            _ => Err(SemanticError::NotAClass {
                ty: object_ty.to_string(),
                span: span.clone(),
            }),
        }
    }

    /// 推导 new 表达式类型
    fn infer_new(
        &self,
        class_name: &str,
        generics: &[Type],
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        // 检查类是否存在（类始终在全局作用域）
        match self.scopes.lookup_global(class_name) {
            Some(Symbol::Class(_)) => {
                if generics.is_empty() {
                    Ok(Type::Class(class_name.to_string()))
                } else {
                    Ok(Type::Generic(class_name.to_string(), generics.to_vec()))
                }
            }
            _ => Err(SemanticError::UndefinedType {
                name: class_name.to_string(),
                span: span.clone(),
            }),
        }
    }

    /// 推导数组字面量类型
    fn infer_array(
        &self,
        elements: &[Expr],
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        if elements.is_empty() {
            // 空数组需要类型注解
            return Err(SemanticError::CannotInferType {
                name: "array literal".to_string(),
                span: span.clone(),
            });
        }

        // 推导第一个元素的类型作为数组元素类型
        let first_ty = self.infer(&elements[0])?;

        // 检查所有元素类型一致
        for elem in elements.iter().skip(1) {
            let elem_ty = self.infer(elem)?;
            if elem_ty != first_ty {
                return Err(SemanticError::TypeMismatch {
                    expected: first_ty.to_string(),
                    found: elem_ty.to_string(),
                    span: elem.span.clone(),
                });
            }
        }

        // 返回固定大小数组类型: [T; N]
        Ok(Type::Array {
            element_type: Box::new(first_ty),
            size: elements.len(),
        })
    }

    /// 推导数组索引类型
    fn infer_index(
        &self,
        array: &Expr,
        index: &Expr,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        let array_ty = self.infer(array)?;
        let index_ty = self.infer(index)?;

        // 索引必须是 int 类型
        if index_ty != Type::Int {
            return Err(SemanticError::TypeMismatch {
                expected: "int".to_string(),
                found: index_ty.to_string(),
                span: index.span.clone(),
            });
        }

        // 🆕 编译期边界检查：如果索引是常量，检查是否越界
        if let ExprKind::Literal(Literal::Int(idx_val)) = &index.kind {
            if let Type::Array { size, .. } = &array_ty {
                // 检查负数索引
                if *idx_val < 0 {
                    return Err(SemanticError::ArrayIndexOutOfBounds {
                        index: *idx_val,
                        size: *size,
                        span: index.span.clone(),
                    });
                }

                // 检查越界
                let idx_usize = *idx_val as usize;
                if idx_usize >= *size {
                    return Err(SemanticError::ArrayIndexOutOfBounds {
                        index: *idx_val,
                        size: *size,
                        span: index.span.clone(),
                    });
                }
            }
        }

        // 数组类型检查
        match &array_ty {
            Type::Array { element_type, .. } => Ok((**element_type).clone()),
            Type::Generic(name, args) if name == "List" && !args.is_empty() => {
                // 动态数组 List<T>
                Ok(args[0].clone())
            }
            _ => Err(SemanticError::TypeMismatch {
                expected: "array or list".to_string(),
                found: array_ty.to_string(),
                span: span.clone(),
            }),
        }
    }
}

/// 检查两个类型是否兼容（用于赋值）
pub fn is_compatible(expected: &Type, actual: &Type) -> bool {
    match (expected, actual) {
        // 完全相同
        (a, b) if a == b => true,

        // int 可以隐式转为 float（Beryl 设计决策：这是唯一允许的隐式转换）
        (Type::Float, Type::Int) => true,

        // 可空类型可以接受非空类型
        (Type::Nullable(inner), actual) => is_compatible(inner, actual),

        // Error 类型用于错误恢复，总是兼容
        (Type::Error, _) | (_, Type::Error) => true,

        _ => false,
    }
}
