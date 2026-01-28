use super::super::TypeInferer;
use crate::error::SemanticError;
use lency_syntax::ast::{Expr, ExprKind, Literal, Type};

impl<'a> TypeInferer<'a> {
    /// 推导数组字面量类型
    pub(crate) fn infer_array_impl(
        &mut self,
        elements: &mut [Expr],
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
        let first_ty = self.infer(&mut elements[0])?;

        // 检查所有元素类型一致
        for elem in elements.iter_mut().skip(1) {
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
    pub(crate) fn infer_index_impl(
        &mut self,
        array: &mut Expr,
        index: &mut Expr,
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

        // 编译期边界检查：如果索引是常量，检查是否越界
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
            Type::Nullable(inner) => match **inner {
                Type::Array {
                    ref element_type, ..
                } => Ok((**element_type).clone()),
                Type::Generic(ref name, ref args) if name == "List" && !args.is_empty() => {
                    Ok(args[0].clone())
                }
                _ => Err(SemanticError::TypeMismatch {
                    expected: "array or list".to_string(),
                    found: array_ty.to_string(),
                    span: span.clone(),
                }),
            },
            Type::Generic(name, args) if name == "List" && !args.is_empty() => {
                // 动态数组 List<T>
                Ok(args[0].clone())
            }
            Type::String => {
                // 字符串索引: s[i] -> int (byte value)
                Ok(Type::Int)
            }
            Type::Vec(inner_type) => {
                // Vector 索引: v[i] -> T
                Ok(*inner_type.clone())
            }
            _ => Err(SemanticError::TypeMismatch {
                expected: "array, list or string".to_string(),
                found: array_ty.to_string(),
                span: span.clone(),
            }),
        }
    }
}
