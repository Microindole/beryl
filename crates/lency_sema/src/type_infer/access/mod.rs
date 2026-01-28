mod index;
mod member;
mod variable;

use super::TypeInferer;
use crate::error::SemanticError;
use lency_syntax::ast::{Expr, Type};

impl<'a> TypeInferer<'a> {
    pub(crate) fn infer_variable(
        &self,
        name: &str,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        self.infer_variable_impl(name, span)
    }

    pub(crate) fn infer_get(
        &mut self,
        object: &mut Expr,
        name: &str,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        self.infer_get_impl(object, name, span)
    }

    pub(crate) fn infer_safe_get(
        &mut self,
        object: &mut Expr,
        name: &str,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        self.infer_safe_get_impl(object, name, span)
    }

    pub(crate) fn infer_array(
        &mut self,
        elements: &mut [Expr],
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        self.infer_array_impl(elements, span)
    }

    pub(crate) fn infer_index(
        &mut self,
        array: &mut Expr,
        index: &mut Expr,
        span: &std::ops::Range<usize>,
    ) -> Result<Type, SemanticError> {
        self.infer_index_impl(array, index, span)
    }
}
