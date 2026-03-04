mod compile;
mod emitter;

pub use compile::compile_lir_to_llvm_ir;

#[cfg(test)]
mod tests;
