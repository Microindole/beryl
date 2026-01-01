use crate::type_infer::TypeInferer;
use beryl_syntax::ast::{Literal, Type};

impl<'a> TypeInferer<'a> {
    /// 推导字面量类型
    pub(crate) fn infer_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::String(_) => Type::String,
            Literal::Null => Type::Nullable(Box::new(Type::Error)), // null 需要上下文推导
        }
    }
}
