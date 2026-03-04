use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValueType {
    I64,
    I1,
}

struct Emitter {
    lines: Vec<String>,
    temp_counter: usize,
    vars: HashSet<String>,
    temps: HashMap<String, ValueType>,
    extern_funcs: HashMap<String, usize>,
    terminated: bool,
}

impl Emitter {
    fn new(vars: HashSet<String>) -> Self {
        Self {
            lines: Vec::new(),
            temp_counter: 0,
            vars,
            temps: HashMap::new(),
            extern_funcs: HashMap::new(),
            terminated: false,
        }
    }

    fn push(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }

    fn next_tmp(&mut self, prefix: &str) -> String {
        let name = format!("%{}_{}", prefix, self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn parse_i64_literal(op: &str) -> Option<i64> {
        op.parse::<i64>().ok()
    }

    fn ensure_i64(&mut self, repr: String, ty: ValueType) -> (String, ValueType) {
        match ty {
            ValueType::I64 => (repr, ValueType::I64),
            ValueType::I1 => {
                let widened = self.next_tmp("zext");
                self.push(format!("  {} = zext i1 {} to i64", widened, repr));
                (widened, ValueType::I64)
            }
        }
    }

    fn ensure_i1(&mut self, repr: String, ty: ValueType) -> (String, ValueType) {
        match ty {
            ValueType::I1 => (repr, ValueType::I1),
            ValueType::I64 => {
                let narrowed = self.next_tmp("to_bool");
                self.push(format!("  {} = icmp ne i64 {}, 0", narrowed, repr));
                (narrowed, ValueType::I1)
            }
        }
    }

    fn emit_operand(&mut self, op: &str) -> Result<(String, ValueType)> {
        if let Some(v) = Self::parse_i64_literal(op) {
            return Ok((v.to_string(), ValueType::I64));
        }
        if !op.starts_with('%') {
            bail!("unsupported operand: {}", op);
        }

        if self.vars.contains(op) {
            let loaded = self.next_tmp("load");
            self.push(format!("  {} = load i64, i64* {}.addr", loaded, op));
            return Ok((loaded, ValueType::I64));
        }

        if let Some(ty) = self.temps.get(op).copied() {
            return Ok((op.to_string(), ty));
        }

        bail!("unknown SSA value: {}", op);
    }

    fn emit_store_var(&mut self, var: &str, repr: String, ty: ValueType) -> Result<()> {
        if !self.vars.contains(var) {
            bail!("unknown variable: {}", var);
        }
        let (repr, _) = self.ensure_i64(repr, ty);
        self.push(format!("  store i64 {}, i64* {}.addr", repr, var));
        Ok(())
    }

    fn mark_temp(&mut self, name: &str, ty: ValueType) {
        self.temps.insert(name.to_string(), ty);
    }

    fn note_extern_func(&mut self, name: &str, argc: usize) -> Result<()> {
        if let Some(prev) = self.extern_funcs.get(name).copied() {
            if prev != argc {
                bail!(
                    "extern function '{}' called with inconsistent arg count: {} vs {}",
                    name,
                    prev,
                    argc
                );
            }
            return Ok(());
        }
        self.extern_funcs.insert(name.to_string(), argc);
        Ok(())
    }

    fn emit_binary(&mut self, dst: &str, op: &str, lhs: &str, rhs: &str) -> Result<()> {
        let (lhs_repr, lhs_ty) = self.emit_operand(lhs)?;
        let (rhs_repr, rhs_ty) = self.emit_operand(rhs)?;

        match op {
            "add" | "sub" | "mul" | "div" => {
                let (lhs_i64, _) = self.ensure_i64(lhs_repr, lhs_ty);
                let (rhs_i64, _) = self.ensure_i64(rhs_repr, rhs_ty);
                let llvm_op = match op {
                    "add" => "add",
                    "sub" => "sub",
                    "mul" => "mul",
                    "div" => "sdiv",
                    _ => unreachable!(),
                };
                self.push(format!(
                    "  {} = {} i64 {}, {}",
                    dst, llvm_op, lhs_i64, rhs_i64
                ));
                self.mark_temp(dst, ValueType::I64);
            }
            "cmp_eq" | "cmp_ne" | "cmp_lt" | "cmp_le" | "cmp_gt" | "cmp_ge" => {
                let (lhs_i64, _) = self.ensure_i64(lhs_repr, lhs_ty);
                let (rhs_i64, _) = self.ensure_i64(rhs_repr, rhs_ty);
                let pred = match op {
                    "cmp_eq" => "eq",
                    "cmp_ne" => "ne",
                    "cmp_lt" => "slt",
                    "cmp_le" => "sle",
                    "cmp_gt" => "sgt",
                    "cmp_ge" => "sge",
                    _ => unreachable!(),
                };
                self.push(format!(
                    "  {} = icmp {} i64 {}, {}",
                    dst, pred, lhs_i64, rhs_i64
                ));
                self.mark_temp(dst, ValueType::I1);
            }
            "and" | "or" => {
                let (lhs_i1, _) = self.ensure_i1(lhs_repr, lhs_ty);
                let (rhs_i1, _) = self.ensure_i1(rhs_repr, rhs_ty);
                let llvm_op = if op == "and" { "and" } else { "or" };
                self.push(format!("  {} = {} i1 {}, {}", dst, llvm_op, lhs_i1, rhs_i1));
                self.mark_temp(dst, ValueType::I1);
            }
            _ => bail!("unsupported binary op: {}", op),
        }
        Ok(())
    }

    fn emit_unary(&mut self, dst: &str, op: &str, rhs: &str) -> Result<()> {
        let (rhs_repr, rhs_ty) = self.emit_operand(rhs)?;
        match op {
            "neg" => {
                let (rhs_i64, _) = self.ensure_i64(rhs_repr, rhs_ty);
                self.push(format!("  {} = sub i64 0, {}", dst, rhs_i64));
                self.mark_temp(dst, ValueType::I64);
            }
            "not" => {
                let (rhs_i1, _) = self.ensure_i1(rhs_repr, rhs_ty);
                self.push(format!("  {} = xor i1 {}, true", dst, rhs_i1));
                self.mark_temp(dst, ValueType::I1);
            }
            _ => bail!("unsupported unary op: {}", op),
        }
        Ok(())
    }
}

fn collect_vars(source: &str) -> Result<HashSet<String>> {
    let mut vars = HashSet::new();
    for raw in source.lines() {
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

/// Compile LIR text emitted by lencyc `--emit-lir` into LLVM IR.
pub fn compile_lir_to_llvm_ir(source: &str) -> Result<String> {
    let vars = collect_vars(source)?;
    let mut emitter = Emitter::new(vars.clone());

    emitter.push("define i32 @main() {");
    emitter.push("entry:");

    for var in &vars {
        emitter.push(format!("  {}.addr = alloca i64", var));
    }

    for raw in source.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line == "func main {" || line == "}" {
            continue;
        }
        if line.ends_with(':') {
            emitter.terminated = false;
            if line == "entry:" {
                continue;
            }
            emitter.push(line.to_string());
            continue;
        }
        if emitter.terminated {
            continue;
        }

        if line.starts_with("var %") {
            let rest = line
                .strip_prefix("var ")
                .ok_or_else(|| anyhow!("invalid var line: {}", line))?;
            let (name, rhs) = rest
                .split_once(" = ")
                .ok_or_else(|| anyhow!("invalid var line: {}", line))?;
            let (rhs_repr, rhs_ty) = emitter.emit_operand(rhs.trim())?;
            emitter.emit_store_var(name.trim(), rhs_repr, rhs_ty)?;
            continue;
        }

        if line.starts_with("store %") {
            let rest = line
                .strip_prefix("store ")
                .ok_or_else(|| anyhow!("invalid store line: {}", line))?;
            let (name, rhs) = rest
                .split_once(", ")
                .ok_or_else(|| anyhow!("invalid store line: {}", line))?;
            let (rhs_repr, rhs_ty) = emitter.emit_operand(rhs.trim())?;
            emitter.emit_store_var(name.trim(), rhs_repr, rhs_ty)?;
            continue;
        }

        if line.starts_with("jmp ") {
            let label = line.trim_start_matches("jmp ").trim();
            emitter.push(format!("  br label %{}", label));
            emitter.terminated = true;
            continue;
        }

        if line.starts_with("br ") {
            let rest = line
                .trim_start_matches("br ")
                .trim()
                .split(", ")
                .collect::<Vec<_>>();
            if rest.len() != 3 {
                bail!("invalid br instruction: {}", line);
            }
            let (cond_repr, cond_ty) = emitter.emit_operand(rest[0].trim())?;
            let (cond_i1, _) = emitter.ensure_i1(cond_repr, cond_ty);
            emitter.push(format!(
                "  br i1 {}, label %{}, label %{}",
                cond_i1,
                rest[1].trim(),
                rest[2].trim()
            ));
            emitter.terminated = true;
            continue;
        }

        if line == "ret" {
            emitter.push("  ret i32 0");
            emitter.terminated = true;
            continue;
        }

        if line.starts_with("ret ") {
            let val = line.trim_start_matches("ret ").trim();
            let (repr, ty) = emitter.emit_operand(val)?;
            match ty {
                ValueType::I64 => {
                    let code = emitter.next_tmp("ret_i32");
                    emitter.push(format!("  {} = trunc i64 {} to i32", code, repr));
                    emitter.push(format!("  ret i32 {}", code));
                }
                ValueType::I1 => {
                    let code = emitter.next_tmp("ret_i32");
                    emitter.push(format!("  {} = zext i1 {} to i32", code, repr));
                    emitter.push(format!("  ret i32 {}", code));
                }
            }
            emitter.terminated = true;
            continue;
        }

        if line.starts_with('%') && line.contains(" = ") {
            let (dst, rhs) = line
                .split_once(" = ")
                .ok_or_else(|| anyhow!("invalid assignment: {}", line))?;
            let dst = dst.trim();
            if rhs == "expr_unknown" {
                emitter.push(format!("  {} = add i64 0, 0", dst));
                emitter.mark_temp(dst, ValueType::I64);
                continue;
            }

            if let Some(rest) = rhs.strip_prefix("call ") {
                if rest == "?()" {
                    emitter.push(format!("  {} = add i64 0, 0", dst));
                    emitter.mark_temp(dst, ValueType::I64);
                    continue;
                }
                let (callee, args_raw) = rest
                    .split_once('(')
                    .ok_or_else(|| anyhow!("invalid call instruction: {}", line))?;
                let args_raw = args_raw
                    .strip_suffix(')')
                    .ok_or_else(|| anyhow!("invalid call instruction: {}", line))?;

                let callee = callee.trim();
                if !callee.starts_with('%') {
                    // FIXME: LIR call lowering currently only supports `%symbol(...)`.
                    bail!("unsupported call callee: {}", callee);
                }

                let callee_name = callee.trim_start_matches('%');
                if callee_name.is_empty() {
                    bail!("invalid call callee: {}", line);
                }

                let mut arg_reprs: Vec<String> = Vec::new();
                let args_text = args_raw.trim();
                if !args_text.is_empty() {
                    for arg in args_text.split(", ") {
                        let (arg_repr, arg_ty) = emitter.emit_operand(arg.trim())?;
                        let (arg_i64, _) = emitter.ensure_i64(arg_repr, arg_ty);
                        arg_reprs.push(arg_i64);
                    }
                }

                emitter.note_extern_func(callee_name, arg_reprs.len())?;
                let args_sig = arg_reprs
                    .iter()
                    .map(|repr| format!("i64 {}", repr))
                    .collect::<Vec<_>>()
                    .join(", ");
                emitter.push(format!(
                    "  {} = call i64 @{}({})",
                    dst, callee_name, args_sig
                ));
                emitter.mark_temp(dst, ValueType::I64);
                continue;
            }

            if let Some(rest) = rhs.strip_prefix("get ") {
                let _ = rest;
                // FIXME: Member access lowering (`get obj.field`) is not implemented in LLVM backend yet.
                bail!("unsupported get in minimal LIR backend: {}", line);
            }

            let parts = rhs.split_whitespace().collect::<Vec<_>>();
            if parts.len() == 2 {
                emitter.emit_unary(dst, parts[0], parts[1])?;
                continue;
            }
            if parts.len() >= 3 {
                let op = parts[0];
                let rhs_joined = rhs
                    .strip_prefix(op)
                    .ok_or_else(|| anyhow!("invalid binary instruction: {}", line))?
                    .trim();
                let (lhs, rhs_val) = rhs_joined
                    .split_once(", ")
                    .ok_or_else(|| anyhow!("invalid binary operands: {}", line))?;
                emitter.emit_binary(dst, op, lhs.trim(), rhs_val.trim())?;
                continue;
            }

            bail!("unsupported assignment form: {}", line);
        }

        bail!("unsupported lir line: {}", line);
    }

    if !emitter.terminated {
        emitter.push("  ret i32 0");
    }
    emitter.push("}");

    let mut out_lines = Vec::new();
    let mut extern_names = emitter.extern_funcs.keys().cloned().collect::<Vec<_>>();
    extern_names.sort();
    for name in extern_names {
        let argc = emitter.extern_funcs.get(&name).copied().unwrap_or(0);
        if argc == 0 {
            out_lines.push(format!("declare i64 @{}()", name));
        } else {
            let args_sig = vec!["i64"; argc].join(", ");
            out_lines.push(format!("declare i64 @{}({})", name, args_sig));
        }
    }
    if !out_lines.is_empty() {
        out_lines.push(String::new());
    }
    out_lines.extend(emitter.lines);

    Ok(format!("{}\n", out_lines.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_min_lir_to_llvm() {
        let src = r#"
; lencyc-lir v0
func main {
entry:
  var %x = 1
  var %y = 2
  %t0 = add %x, %y
  store %x, %t0
  %t1 = cmp_gt %x, 0
  br %t1, then_0, else_1
then_0:
  ret %x
else_1:
  ret 0
}
"#;
        let result = compile_lir_to_llvm_ir(src);
        assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
        let ir = result.unwrap_or_default();
        assert!(ir.contains("define i32 @main()"));
        assert!(ir.contains("alloca i64"));
        assert!(ir.contains("icmp sgt i64"));
        assert!(ir.contains("ret i32"));
    }

    #[test]
    fn test_compile_lir_call_external_function() {
        let src = r#"
; lencyc-lir v0
func main {
entry:
  var %x = 5
  %t0 = call %foo(%x, 1)
  ret %t0
}
"#;
        let result = compile_lir_to_llvm_ir(src);
        assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
        let ir = result.unwrap_or_default();
        assert!(ir.contains("declare i64 @foo(i64, i64)"));
        assert!(ir.contains("call i64 @foo("));
    }
}
