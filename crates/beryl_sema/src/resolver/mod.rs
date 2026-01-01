//! Name Resolver
//!
//! 名称解析 Pass，收集所有定义并解析标识符引用。
//! 这是语义分析的第一步，为后续类型检查奠定基础。

use crate::error::SemanticError;
use crate::scope::ScopeStack;
use beryl_syntax::ast::{Decl, Expr, Program, Stmt};

pub mod decl;
pub mod expr;
pub mod stmt;

/// 名称解析器
pub struct Resolver {
    pub(crate) scopes: ScopeStack,
    pub(crate) errors: Vec<SemanticError>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: ScopeStack::new(),
            errors: Vec::new(),
        }
    }

    /// 解析整个程序
    ///
    /// 采用两遍扫描：
    /// 1. 第一遍：收集所有顶层声明（函数、类）
    /// 2. 第二遍：解析函数体内的引用
    pub fn resolve(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        // Pass 1: 收集顶层声明
        for decl in &program.decls {
            self.collect_decl(decl);
        }

        // Pass 2: 解析函数体
        for decl in &program.decls {
            self.resolve_decl(decl);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    // --- Delegation ---

    pub(crate) fn collect_decl(&mut self, decl: &Decl) {
        decl::collect_decl(self, decl);
    }

    pub(crate) fn resolve_decl(&mut self, decl: &Decl) {
        decl::resolve_decl(self, decl);
    }

    pub(crate) fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt::resolve_stmt(self, stmt);
    }

    pub(crate) fn resolve_expr(&mut self, expr: &Expr) {
        expr::resolve_expr(self, expr);
    }

    // --- Accessors ---

    /// 获取作用域栈的引用
    pub fn scopes(&self) -> &ScopeStack {
        &self.scopes
    }

    /// 获取作用域栈的所有权
    pub fn into_scopes(self) -> ScopeStack {
        self.scopes
    }

    /// 获取收集到的错误
    pub fn errors(&self) -> &[SemanticError] {
        &self.errors
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
