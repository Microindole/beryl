use super::Specializer;
use lency_syntax::ast::{Decl, EnumVariant, Field, GenericParam, Param};

pub fn specialize(spec: &Specializer, decl: &Decl) -> Decl {
    match decl {
        Decl::Var {
            span,
            name,
            ty,
            value,
        } => Decl::Var {
            span: span.clone(),
            name: name.clone(),
            ty: ty.clone(),       // Should specialize type
            value: value.clone(), // Should specialize expr
        },
        Decl::Import { items, span } => Decl::Import {
            span: span.clone(),
            items: items.clone(),
        },
        Decl::Struct {
            span,
            name,
            generic_params,
            fields,
        } => {
            // Keep generic_params that are NOT in type_map
            let remaining_params: Vec<GenericParam> = generic_params
                .iter()
                .filter(|p| !spec.type_map.contains_key(&p.name))
                .cloned()
                .collect();

            Decl::Struct {
                span: span.clone(),
                name: name.clone(),
                generic_params: remaining_params,
                fields: fields.iter().map(|f| spec.specialize_field(f)).collect(),
            }
        }
        Decl::Function {
            span,
            name,
            generic_params,
            params,
            return_type,
            body,
        } => {
            let remaining_params: Vec<GenericParam> = generic_params
                .iter()
                .filter(|p| !spec.type_map.contains_key(&p.name))
                .cloned()
                .collect();

            Decl::Function {
                span: span.clone(),
                name: name.clone(),
                generic_params: remaining_params,
                params: params.iter().map(|p| spec.specialize_param(p)).collect(),
                return_type: spec.specialize_type(return_type),
                body: body.iter().map(|stmt| spec.specialize_stmt(stmt)).collect(),
            }
        }
        Decl::ExternFunction {
            span,
            name,
            generic_params,
            params,
            return_type,
        } => {
            let remaining_params: Vec<GenericParam> = generic_params
                .iter()
                .filter(|p| !spec.type_map.contains_key(&p.name))
                .cloned()
                .collect();

            Decl::ExternFunction {
                span: span.clone(),
                name: name.clone(),
                generic_params: remaining_params,
                params: params.iter().map(|p| spec.specialize_param(p)).collect(),
                return_type: spec.specialize_type(return_type),
            }
        }
        Decl::Impl {
            span,
            trait_ref,
            type_name,
            generic_params,
            methods,
        } => {
            let remaining_params: Vec<GenericParam> = generic_params
                .iter()
                .filter(|p| !spec.type_map.contains_key(&p.name))
                .cloned()
                .collect();

            Decl::Impl {
                span: span.clone(),
                trait_ref: trait_ref.clone(),
                type_name: type_name.clone(),
                generic_params: remaining_params,
                methods: methods.iter().map(|m| spec.specialize_decl(m)).collect(),
            }
        }
        // Trait 定义：目前不需要特化，直接保留
        Decl::Trait {
            span,
            name,
            generic_params,
            methods,
        } => Decl::Trait {
            span: span.clone(),
            name: name.clone(),
            generic_params: generic_params.clone(),
            methods: methods.clone(),
        },
        Decl::Enum {
            span,
            name,
            generic_params,
            variants,
        } => {
            let remaining_params: Vec<GenericParam> = generic_params
                .iter()
                .filter(|p| !spec.type_map.contains_key(&p.name))
                .cloned()
                .collect();

            Decl::Enum {
                span: span.clone(),
                name: name.clone(),
                generic_params: remaining_params,
                variants: variants
                    .iter()
                    .map(|v| match v {
                        EnumVariant::Unit(n) => EnumVariant::Unit(n.clone()),
                        EnumVariant::Tuple(n, types) => EnumVariant::Tuple(
                            n.clone(),
                            types.iter().map(|t| spec.specialize_type(t)).collect(),
                        ),
                    })
                    .collect(),
            }
        }
    }
}

pub fn specialize_field(spec: &Specializer, field: &Field) -> Field {
    Field {
        name: field.name.clone(),
        ty: spec.specialize_type(&field.ty),
    }
}

pub fn specialize_param(spec: &Specializer, param: &Param) -> Param {
    Param {
        name: param.name.clone(),
        ty: spec.specialize_type(&param.ty),
    }
}
