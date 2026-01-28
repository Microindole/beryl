//! Generic Type Specializer
//!
//! 负责将 AST 中的泛型参数替换为具体类型。
//! 这是一个 AST -> AST 的变换器。

mod decl;
mod expr;
mod stmt;
mod types;

use lency_syntax::ast::*;
use std::collections::HashMap;

pub struct Specializer {
    /// 泛型参数名到具体类型的映射 (e.g., T -> int)
    pub(crate) type_map: HashMap<String, Type>,
}

impl Specializer {
    pub fn new(type_map: HashMap<String, Type>) -> Self {
        Self { type_map }
    }

    /// 特化类型
    pub fn specialize_type(&self, ty: &Type) -> Type {
        types::specialize(self, ty)
    }

    /// 特化声明
    pub fn specialize_decl(&self, decl: &Decl) -> Decl {
        decl::specialize(self, decl)
    }

    pub fn specialize_field(&self, field: &Field) -> Field {
        decl::specialize_field(self, field)
    }

    pub fn specialize_param(&self, param: &Param) -> Param {
        decl::specialize_param(self, param)
    }

    pub fn specialize_stmt(&self, stmt: &Stmt) -> Stmt {
        stmt::specialize(self, stmt)
    }

    pub fn specialize_expr(&self, expr: &Expr) -> Expr {
        expr::specialize(self, expr)
    }
}
