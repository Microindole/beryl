//! Lency Runtime Library
//!
//! 提供 Lency 语言的运行时支持，包括动态数组、文件 I/O 和字符串处理

pub mod file;
pub mod string;
use std::alloc::{alloc, dealloc, realloc, Layout};

/// Lency 动态数组
#[repr(C)]
pub struct LencyVec {
    data: *mut i64,
    len: i64,
    capacity: i64,
}

impl LencyVec {
    /// 创建新的 Vec
    pub fn new(initial_capacity: i64) -> Box<Self> {
        let capacity = if initial_capacity > 0 {
            initial_capacity
        } else {
            4
        };
        let layout = Layout::array::<i64>(capacity as usize).unwrap();

        let data = unsafe { alloc(layout) as *mut i64 };
        if data.is_null() {
            panic!("Failed to allocate Vec");
        }

        Box::new(LencyVec {
            data,
            len: 0,
            capacity,
        })
    }

    /// 扩容
    fn grow(&mut self) {
        let new_capacity = self.capacity * 2;
        let old_layout = Layout::array::<i64>(self.capacity as usize).unwrap();
        let new_layout = Layout::array::<i64>(new_capacity as usize).unwrap();

        let new_data =
            unsafe { realloc(self.data as *mut u8, old_layout, new_layout.size()) as *mut i64 };

        if new_data.is_null() {
            panic!("Failed to grow Vec");
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }

    /// 添加元素
    pub fn push(&mut self, element: i64) {
        if self.len >= self.capacity {
            self.grow();
        }
        unsafe {
            *self.data.offset(self.len as isize) = element;
        }
        self.len += 1;
    }

    /// 弹出元素
    pub fn pop(&mut self) -> i64 {
        if self.len == 0 {
            eprintln!("Cannot pop from empty Vec");
            return 0;
        }
        self.len -= 1;
        unsafe { *self.data.offset(self.len as isize) }
    }

    /// 获取长度
    pub fn len(&self) -> i64 {
        self.len
    }

    /// 检查是否为空
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 获取元素
    pub fn get(&self, index: i64) -> i64 {
        if index < 0 || index >= self.len {
            panic!("Vec index out of bounds: {} (len: {})", index, self.len);
        }
        unsafe { *self.data.offset(index as isize) }
    }

    /// 设置元素
    pub fn set(&mut self, index: i64, value: i64) {
        if index < 0 || index >= self.len {
            panic!("Vec index out of bounds: {} (len: {})", index, self.len);
        }
        unsafe {
            *self.data.offset(index as isize) = value;
        }
    }
}

impl Drop for LencyVec {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = Layout::array::<i64>(self.capacity as usize).unwrap();
            unsafe {
                dealloc(self.data as *mut u8, layout);
            }
        }
    }
}

// C-compatible FFI functions

/// Create a new Vec
#[no_mangle]
pub extern "C" fn lency_vec_new(initial_capacity: i64) -> *mut LencyVec {
    Box::into_raw(LencyVec::new(initial_capacity))
}

/// Push an element to the Vec
///
/// # Safety
/// `vec` must be a valid pointer returned by `lency_vec_new`
#[no_mangle]
pub unsafe extern "C" fn lency_vec_push(vec: *mut LencyVec, element: i64) {
    unsafe {
        if !vec.is_null() {
            (*vec).push(element);
        }
    }
}

/// Pop an element from the Vec
///
/// # Safety  
/// `vec` must be a valid pointer returned by `lency_vec_new`
#[no_mangle]
pub unsafe extern "C" fn lency_vec_pop(vec: *mut LencyVec) -> i64 {
    unsafe {
        if vec.is_null() {
            return 0;
        }
        (*vec).pop()
    }
}

/// Get the length of the Vec
///
/// # Safety
/// `vec` must be a valid pointer returned by `lency_vec_new`
#[no_mangle]
pub unsafe extern "C" fn lency_vec_len(vec: *const LencyVec) -> i64 {
    unsafe {
        if vec.is_null() {
            return 0;
        }
        (*vec).len()
    }
}

/// Get an element from the Vec
///
/// # Safety
/// `vec` must be a valid pointer returned by `lency_vec_new`
#[no_mangle]
pub unsafe extern "C" fn lency_vec_get(vec: *const LencyVec, index: i64) -> i64 {
    unsafe {
        if vec.is_null() {
            return 0;
        }
        (*vec).get(index)
    }
}

/// Set an element in the Vec
///
/// # Safety
/// `vec` must be a valid pointer returned by `lency_vec_new`
#[no_mangle]
pub unsafe extern "C" fn lency_vec_set(vec: *mut LencyVec, index: i64, value: i64) {
    unsafe {
        if !vec.is_null() {
            (*vec).set(index, value);
        }
    }
}

/// Free a Vec
///
/// # Safety
/// `vec` must be a valid pointer returned by `lency_vec_new` and not already freed
#[no_mangle]
pub unsafe extern "C" fn lency_vec_free(vec: *mut LencyVec) {
    if !vec.is_null() {
        unsafe {
            let _ = Box::from_raw(vec);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_new() {
        let vec = LencyVec::new(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity, 10);
    }

    #[test]
    fn test_vec_push_pop() {
        let mut vec = LencyVec::new(2);
        vec.push(1);
        vec.push(2);
        vec.push(3); // 触发扩容

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.pop(), 3);
        assert_eq!(vec.pop(), 2);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_vec_get_set() {
        let mut vec = LencyVec::new(5);
        vec.push(10);
        vec.push(20);

        assert_eq!(vec.get(0), 10);
        assert_eq!(vec.get(1), 20);

        vec.set(0, 99);
        assert_eq!(vec.get(0), 99);
    }
}
