//! Result Code Generation
//!
//! Result<T, E> 相关的代码生成：构造、?操作符、内置方法

mod constructor;
mod methods;
mod try_op;

pub use constructor::{gen_err, gen_ok};
pub use methods::gen_result_builtin_method;
pub use try_op::gen_try;
