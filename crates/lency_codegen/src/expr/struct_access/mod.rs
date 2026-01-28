//! Struct Access Code Generation
//!
//! 处理结构体成员访问：point.x

mod access;
mod common;
mod ptr;
mod safe;

pub use access::gen_member_access;
pub use ptr::gen_struct_member_ptr;
pub use safe::gen_safe_member_access;

// Optional: export gen_struct_member_ptr_val if used outside?
// It was public in original file.
// pub use ptr::gen_struct_member_ptr_val;
