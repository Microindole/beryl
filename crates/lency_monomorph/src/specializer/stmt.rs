use super::Specializer;
use lency_syntax::ast::Stmt;

pub fn specialize(spec: &Specializer, stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::VarDecl {
            span,
            name,
            ty,
            value,
        } => Stmt::VarDecl {
            span: span.clone(),
            name: name.clone(),
            ty: ty.as_ref().map(|t| spec.specialize_type(t)),
            value: spec.specialize_expr(value),
        },
        Stmt::Assignment {
            span,
            target,
            value,
        } => Stmt::Assignment {
            span: span.clone(),
            target: spec.specialize_expr(target),
            value: spec.specialize_expr(value),
        },
        Stmt::Expression(expr) => Stmt::Expression(spec.specialize_expr(expr)),
        Stmt::Block(stmts) => Stmt::Block(stmts.iter().map(|s| spec.specialize_stmt(s)).collect()),
        Stmt::Return { span, value } => Stmt::Return {
            span: span.clone(),
            value: value.as_ref().map(|e| spec.specialize_expr(e)),
        },
        Stmt::If {
            span,
            condition,
            then_block,
            else_block,
        } => Stmt::If {
            span: span.clone(),
            condition: spec.specialize_expr(condition),
            then_block: then_block.iter().map(|s| spec.specialize_stmt(s)).collect(),
            else_block: else_block
                .as_ref()
                .map(|b| b.iter().map(|s| spec.specialize_stmt(s)).collect()),
        },
        Stmt::While {
            span,
            condition,
            body,
        } => Stmt::While {
            span: span.clone(),
            condition: spec.specialize_expr(condition),
            body: body.iter().map(|s| spec.specialize_stmt(s)).collect(),
        },
        Stmt::For {
            span,
            init,
            condition,
            update,
            body,
        } => Stmt::For {
            span: span.clone(),
            init: init.as_ref().map(|s| Box::new(spec.specialize_stmt(s))),
            condition: condition.as_ref().map(|e| spec.specialize_expr(e)),
            update: update.as_ref().map(|s| Box::new(spec.specialize_stmt(s))),
            body: body.iter().map(|s| spec.specialize_stmt(s)).collect(),
        },
        Stmt::ForIn {
            span,
            iterator,
            iterable,
            body,
        } => Stmt::ForIn {
            span: span.clone(),
            iterator: iterator.clone(),
            iterable: spec.specialize_expr(iterable),
            body: body.iter().map(|s| spec.specialize_stmt(s)).collect(),
        },
        Stmt::Break { span } => Stmt::Break { span: span.clone() },
        Stmt::Continue { span } => Stmt::Continue { span: span.clone() },
    }
}
