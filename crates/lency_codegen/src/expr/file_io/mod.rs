//! File I/O Code Generation
//!
//! 文件 I/O 操作的代码生成：read_file, write_file

mod ffi;
mod read;
mod write;

pub use read::gen_read_file;
pub use write::gen_write_file;
