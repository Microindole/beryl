use super::{is_compatible, TypeInferer};
use crate::error::SemanticError;
use beryl_syntax::ast::{Expr, MatchCase, MatchPattern, Type};

impl<'a> TypeInferer<'a> {
    pub(crate) fn infer_match(
        &self,
        value: &Expr,
        cases: &[MatchCase],
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
                MatchPattern::Literal(lit) => {
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
}
