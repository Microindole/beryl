use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};

use super::super::emitter::{llvm_type_str, Emitter, ExternSig, ValueType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct LirFunction {
    pub(super) name: String,
    pub(super) params: Vec<(String, ValueType)>,
    pub(super) ret_ty: ValueType,
    pub(super) body_lines: Vec<String>,
}

pub(super) fn parse_value_type(raw: &str) -> Result<ValueType> {
    match raw.trim() {
        "i64" => Ok(ValueType::I64),
        "i1" => Ok(ValueType::I1),
        "ptr" => Ok(ValueType::Ptr),
        "void" => Ok(ValueType::Void),
        other => bail!("unsupported lir value type: {}", other),
    }
}

pub(super) fn parse_functions(source: &str) -> Result<Vec<LirFunction>> {
    let mut functions = Vec::new();
    let lines = source.lines().collect::<Vec<_>>();
    let mut idx = 0;
    while idx < lines.len() {
        let line = lines[idx].trim();
        if line.is_empty() || line.starts_with(';') {
            idx += 1;
            continue;
        }
        if !line.starts_with("func ") {
            bail!("unsupported top-level lir line: {}", line);
        }

        let header = line
            .strip_prefix("func ")
            .ok_or_else(|| anyhow!("invalid function header: {}", line))?;
        let (name, params_raw, ret_ty) = if header.contains(" -> ") {
            let (name_and_params, ret_raw) = header
                .split_once(" -> ")
                .ok_or_else(|| anyhow!("invalid function header: {}", line))?;
            let ret_raw = ret_raw
                .strip_suffix('{')
                .ok_or_else(|| anyhow!("invalid function header: {}", line))?
                .trim();
            let ret_ty = parse_value_type(ret_raw)?;
            let (name, params_raw) = name_and_params
                .split_once('(')
                .ok_or_else(|| anyhow!("invalid function header: {}", line))?;
            let params_raw = params_raw
                .strip_suffix(')')
                .ok_or_else(|| anyhow!("invalid function header: {}", line))?;
            (
                name.trim().to_string(),
                params_raw.trim().to_string(),
                ret_ty,
            )
        } else {
            let legacy_name = header
                .strip_suffix('{')
                .ok_or_else(|| anyhow!("invalid legacy function header: {}", line))?
                .trim();
            (legacy_name.to_string(), String::new(), ValueType::I64)
        };
        let mut params = Vec::new();
        if !params_raw.trim().is_empty() {
            for part in params_raw.split(", ") {
                let (param_name, param_ty_raw) = part
                    .split_once(": ")
                    .ok_or_else(|| anyhow!("invalid function param: {}", part))?;
                if !param_name.starts_with('%') {
                    bail!("invalid function param name: {}", param_name);
                }
                params.push((
                    param_name.trim().to_string(),
                    parse_value_type(param_ty_raw)?,
                ));
            }
        }

        idx += 1;
        let mut body_lines = Vec::new();
        while idx < lines.len() {
            let body_line = lines[idx].trim();
            idx += 1;
            if body_line == "}" {
                break;
            }
            body_lines.push(body_line.to_string());
        }
        functions.push(LirFunction {
            name,
            params,
            ret_ty,
            body_lines,
        });
    }

    Ok(functions)
}

pub(super) fn collect_vars(lines: &[String]) -> Result<HashSet<String>> {
    let mut vars = HashSet::new();
    for raw in lines {
        let line = raw.trim();
        if line.starts_with("var %") {
            let rest = line
                .strip_prefix("var ")
                .ok_or_else(|| anyhow!("invalid var line: {}", line))?;
            let (name, _) = rest
                .split_once(" = ")
                .ok_or_else(|| anyhow!("invalid var assignment: {}", line))?;
            vars.insert(name.trim().to_string());
        } else if line.starts_with("store %") {
            let rest = line
                .strip_prefix("store ")
                .ok_or_else(|| anyhow!("invalid store line: {}", line))?;
            let (name, _) = rest
                .split_once(", ")
                .ok_or_else(|| anyhow!("invalid store assignment: {}", line))?;
            vars.insert(name.trim().to_string());
        }
    }
    Ok(vars)
}

pub(super) fn build_function_signatures(
    functions: &[LirFunction],
) -> HashMap<String, (Vec<ValueType>, ValueType)> {
    let mut out = HashMap::new();
    for func in functions {
        out.insert(
            func.name.clone(),
            (
                func.params.iter().map(|(_, ty)| *ty).collect::<Vec<_>>(),
                func.ret_ty,
            ),
        );
    }
    out
}

pub(super) fn llvm_function_ret_ty(name: &str, ret_ty: ValueType) -> &'static str {
    if name == "main" {
        return "i32";
    }
    llvm_type_str(ret_ty)
}

pub(super) fn resolve_builtin_call(
    callee_name: &str,
) -> Option<(&'static str, Vec<ValueType>, ValueType)> {
    // 当前仅映射 runtime ABI 已稳定的 builtin 子集。
    match callee_name {
        "arg_count" => Some(("lency_arg_count", vec![], ValueType::I64)),
        "arg_at" => Some(("lency_arg_at", vec![ValueType::I64], ValueType::Ptr)),
        "int_to_string" => Some(("lency_int_to_string", vec![ValueType::I64], ValueType::Ptr)),
        "file_exists" => Some(("lency_file_exists", vec![ValueType::Ptr], ValueType::I64)),
        "is_dir" => Some(("lency_file_is_dir", vec![ValueType::Ptr], ValueType::I64)),
        "lency_vec_new" => Some(("lency_vec_new", vec![ValueType::I64], ValueType::Ptr)),
        "lency_vec_push" => Some((
            "lency_vec_push",
            vec![ValueType::Ptr, ValueType::I64],
            ValueType::Void,
        )),
        "lency_vec_get" => Some((
            "lency_vec_get",
            vec![ValueType::Ptr, ValueType::I64],
            ValueType::I64,
        )),
        "lency_vec_set" => Some((
            "lency_vec_set",
            vec![ValueType::Ptr, ValueType::I64, ValueType::I64],
            ValueType::Void,
        )),
        _ => None,
    }
}

pub(super) fn build_output_ir(emitter: Emitter) -> String {
    let mut out_lines = Vec::new();
    let mut string_globals = emitter
        .string_globals
        .values()
        .map(|(_, _, decl)| decl.clone())
        .collect::<Vec<_>>();
    string_globals.sort();
    out_lines.extend(string_globals);
    if !out_lines.is_empty() {
        out_lines.push(String::new());
    }
    let mut extern_names = emitter.extern_funcs.keys().cloned().collect::<Vec<_>>();
    extern_names.sort();

    for name in extern_names {
        let sig = emitter
            .extern_funcs
            .get(&name)
            .cloned()
            .unwrap_or(ExternSig {
                arg_tys: vec![],
                ret_ty: ValueType::I64,
            });
        if sig.arg_tys.is_empty() {
            out_lines.push(format!("declare {} @{}()", llvm_type_str(sig.ret_ty), name));
        } else {
            let args_sig = sig
                .arg_tys
                .iter()
                .map(|ty| llvm_type_str(*ty).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            out_lines.push(format!(
                "declare {} @{}({})",
                llvm_type_str(sig.ret_ty),
                name,
                args_sig
            ));
        }
    }
    if !out_lines.is_empty() {
        out_lines.push(String::new());
    }
    out_lines.extend(emitter.lines);

    format!("{}\n", out_lines.join("\n"))
}
