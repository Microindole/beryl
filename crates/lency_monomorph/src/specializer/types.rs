use super::Specializer;
use lency_syntax::ast::Type;

pub fn specialize(spec: &Specializer, ty: &Type) -> Type {
    match ty {
        // T -> int
        Type::GenericParam(name) => {
            if let Some(concrete_ty) = spec.type_map.get(name) {
                concrete_ty.clone()
            } else {
                // 如果找不到映射，可能是外层泛型（嵌套函数）或者未被替换
                // 保持原样
                Type::GenericParam(name.clone())
            }
        }

        // Parser 将 T 解析为 Type::Struct("T")，所以我们也需要在 Struct 变体中检查替换
        Type::Struct(name) => {
            if let Some(concrete_ty) = spec.type_map.get(name) {
                concrete_ty.clone()
            } else {
                Type::Struct(name.clone())
            }
        }

        // Box<T> -> Box<int>
        Type::Generic(name, args) => {
            let new_args = args.iter().map(|arg| spec.specialize_type(arg)).collect();
            Type::Generic(name.clone(), new_args)
        }

        Type::Vec(inner) => Type::Vec(Box::new(spec.specialize_type(inner))),

        Type::Array { element_type, size } => Type::Array {
            element_type: Box::new(spec.specialize_type(element_type)),
            size: *size,
        },

        Type::Nullable(inner) => Type::Nullable(Box::new(spec.specialize_type(inner))),

        // 基础类型不变
        _ => ty.clone(),
    }
}
