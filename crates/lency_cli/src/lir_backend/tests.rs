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

#[test]
fn test_compile_lir_maps_arg_count_runtime_symbol() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %arg_count()
  ret %t0
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare i64 @lency_arg_count()"));
    assert!(ir.contains("call i64 @lency_arg_count()"));
}

#[test]
fn test_compile_lir_maps_arg_at_runtime_symbol() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %arg_at(0)
  ret %t0
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare ptr @lency_arg_at(i64)"));
    assert!(ir.contains("call ptr @lency_arg_at(i64 0)"));
}

#[test]
fn test_compile_lir_maps_int_to_string_runtime_symbol() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %int_to_string(7)
  ret %t0
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare ptr @lency_int_to_string(i64)"));
    assert!(ir.contains("call ptr @lency_int_to_string(i64 7)"));
}

#[test]
fn test_compile_lir_get_to_string_lowering() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  var %x = 7
  %t0 = get %x.to_string
  %t1 = call %t0()
  ret %t1
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare ptr @lency_int_to_string(i64)"));
    assert!(ir.contains("call ptr @lency_int_to_string(i64"));
}

#[test]
fn test_compile_lir_call_through_member_temp_no_args() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  var %x = 7
  %t0 = get %x.to_string
  %t1 = call %t0()
  ret %t1
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare ptr @lency_int_to_string(i64)"));
    assert!(ir.contains("call ptr @lency_int_to_string(i64"));
    assert!(!ir.contains("call i64 @t0("));
}

#[test]
fn test_compile_lir_get_len_lowering() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %int_to_string(7)
  %t1 = get %t0.len
  %t2 = call %t1()
  ret %t2
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare i64 @lency_string_len(ptr)"));
    assert!(ir.contains("call i64 @lency_string_len(ptr"));
}

#[test]
fn test_compile_lir_call_member_substr_lowering() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %int_to_string(12345)
  %t1 = get %t0.substr
  %t2 = call %t1(1, 2)
  ret %t2
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare ptr @lency_string_substr(ptr, i64, i64)"));
    assert!(ir.contains("call ptr @lency_string_substr(ptr"));
}

#[test]
fn test_compile_lir_call_member_split_lowering() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %int_to_string(12345)
  %t1 = call %int_to_string(3)
  %t2 = get %t0.split
  %t3 = call %t2(%t1)
  ret %t3
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare ptr @lency_string_split(ptr, ptr)"));
    assert!(ir.contains("call ptr @lency_string_split(ptr"));
}

#[test]
fn test_compile_lir_call_member_generic_fallback() {
    let src = r#"
; lencyc-lir v0
func main {
entry:
  %t0 = call %int_to_string(12345)
  %t1 = call %int_to_string(5)
  %t2 = get %t0.contains
  %t3 = call %t2(%t1)
  ret %t3
}
"#;
    let result = compile_lir_to_llvm_ir(src);
    assert!(result.is_ok(), "lir compile failed: {:?}", result.err());
    let ir = result.unwrap_or_default();
    assert!(ir.contains("declare i1 @contains(ptr, ptr)"));
    assert!(ir.contains("call i1 @contains(ptr"));
}
