pub mod ffi;
pub mod int;
pub mod methods;
pub mod string;

// Re-export main function for backward compatibility
pub use methods::gen_hashmap_extern_call;

/// Check if a function name corresponds to a HashMap extern call
pub fn is_hashmap_extern(name: &str) -> bool {
    name.starts_with("hashmap_")
}
