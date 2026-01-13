//! Lency String Runtime
//!
//! 提供 Lency 语言的字符串处理运行时支持

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::LencyVec;

/// 获取字符串长度
///
/// # Safety
/// `ptr` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn lency_string_len(ptr: *const c_char) -> i64 {
    if ptr.is_null() {
        return 0;
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };
    c_str.to_bytes().len() as i64
}

/// 去除字符串首尾空白
/// 返回新分配的字符串
///
/// # Safety
/// `ptr` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn lency_string_trim(ptr: *const c_char) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let trimmed = s.trim();

    // 分配新内存并复制
    let len = trimmed.len();
    let result = unsafe { libc::malloc(len + 1) as *mut c_char };
    if result.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        std::ptr::copy_nonoverlapping(trimmed.as_ptr(), result as *mut u8, len);
        *result.add(len) = 0; // null terminator
    }

    result
}

/// 按分隔符拆分字符串
/// 返回 LencyVec (存储字符串指针)
///
/// # Safety
/// `str_ptr` and `delim_ptr` must be valid null-terminated C strings
#[no_mangle]
pub unsafe extern "C" fn lency_string_split(
    str_ptr: *const c_char,
    delim_ptr: *const c_char,
) -> *mut LencyVec {
    if str_ptr.is_null() || delim_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(str_ptr) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let c_delim = unsafe { CStr::from_ptr(delim_ptr) };
    let delim = match c_delim.to_str() {
        Ok(d) => d,
        Err(_) => return std::ptr::null_mut(),
    };

    let parts: Vec<&str> = s.split(delim).collect();
    let vec = Box::into_raw(LencyVec::new(parts.len() as i64));

    for part in parts {
        // 分配每个子串
        let len = part.len();
        let part_ptr = unsafe { libc::malloc(len + 1) as *mut c_char };
        if !part_ptr.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(part.as_ptr(), part_ptr as *mut u8, len);
                *part_ptr.add(len) = 0;
                // 将指针作为 i64 存储 (因为 LencyVec 存储 i64)
                (*vec).push(part_ptr as i64);
            }
        }
    }

    vec
}

/// 用分隔符连接字符串数组
/// 返回新分配的字符串
///
/// # Safety
/// `vec_ptr` must be a valid LencyVec containing string pointers
/// `sep_ptr` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn lency_string_join(
    vec_ptr: *const LencyVec,
    sep_ptr: *const c_char,
) -> *mut c_char {
    if vec_ptr.is_null() || sep_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_sep = unsafe { CStr::from_ptr(sep_ptr) };
    let sep = match c_sep.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let vec = unsafe { &*vec_ptr };
    let len = vec.len();

    if len == 0 {
        // 返回空字符串
        let result = unsafe { libc::malloc(1) as *mut c_char };
        if !result.is_null() {
            unsafe { *result = 0 };
        }
        return result;
    }

    // 收集所有字符串
    let mut parts: Vec<String> = Vec::new();
    for i in 0..len {
        let str_ptr = vec.get(i) as *const c_char;
        if !str_ptr.is_null() {
            let c_str = unsafe { CStr::from_ptr(str_ptr) };
            if let Ok(s) = c_str.to_str() {
                parts.push(s.to_string());
            }
        }
    }

    let joined = parts.join(sep);
    let result_len = joined.len();
    let result = unsafe { libc::malloc(result_len + 1) as *mut c_char };
    if result.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        std::ptr::copy_nonoverlapping(joined.as_ptr(), result as *mut u8, result_len);
        *result.add(result_len) = 0;
    }

    result
}

/// 提取子串
/// 返回新分配的字符串
///
/// # Safety
/// `ptr` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn lency_string_substr(
    ptr: *const c_char,
    start: i64,
    len: i64,
) -> *mut c_char {
    if ptr.is_null() || start < 0 || len < 0 {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let start_idx = start as usize;
    let end_idx = std::cmp::min(start_idx + len as usize, s.len());

    if start_idx >= s.len() {
        // 返回空字符串
        let result = unsafe { libc::malloc(1) as *mut c_char };
        if !result.is_null() {
            unsafe { *result = 0 };
        }
        return result;
    }

    let substr = &s[start_idx..end_idx];
    let substr_len = substr.len();
    let result = unsafe { libc::malloc(substr_len + 1) as *mut c_char };
    if result.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        std::ptr::copy_nonoverlapping(substr.as_ptr(), result as *mut u8, substr_len);
        *result.add(substr_len) = 0;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_string_len() {
        let s = CString::new("hello").unwrap();
        assert_eq!(unsafe { lency_string_len(s.as_ptr()) }, 5);

        let empty = CString::new("").unwrap();
        assert_eq!(unsafe { lency_string_len(empty.as_ptr()) }, 0);
    }

    #[test]
    fn test_string_trim() {
        let s = CString::new("  hello world  ").unwrap();
        let result = unsafe { lency_string_trim(s.as_ptr()) };
        assert!(!result.is_null());

        let trimmed = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
        assert_eq!(trimmed, "hello world");

        unsafe { libc::free(result as *mut libc::c_void) };
    }

    #[test]
    fn test_string_split() {
        let s = CString::new("a,b,c").unwrap();
        let delim = CString::new(",").unwrap();

        let vec = unsafe { lency_string_split(s.as_ptr(), delim.as_ptr()) };
        assert!(!vec.is_null());

        unsafe {
            assert_eq!((*vec).len(), 3);

            let part0 = CStr::from_ptr((*vec).get(0) as *const c_char)
                .to_str()
                .unwrap();
            assert_eq!(part0, "a");

            let part1 = CStr::from_ptr((*vec).get(1) as *const c_char)
                .to_str()
                .unwrap();
            assert_eq!(part1, "b");

            let part2 = CStr::from_ptr((*vec).get(2) as *const c_char)
                .to_str()
                .unwrap();
            assert_eq!(part2, "c");

            // 清理
            for i in 0..(*vec).len() {
                libc::free((*vec).get(i) as *mut libc::c_void);
            }
            let _ = Box::from_raw(vec);
        }
    }

    #[test]
    fn test_string_join() {
        // 创建 vec 并填充
        let vec = Box::into_raw(LencyVec::new(3));
        let parts = ["hello", "world", "test"];

        unsafe {
            for part in &parts {
                let cs = CString::new(*part).unwrap();
                let ptr = libc::malloc(part.len() + 1) as *mut c_char;
                std::ptr::copy_nonoverlapping(cs.as_ptr(), ptr, part.len() + 1);
                (*vec).push(ptr as i64);
            }

            let sep = CString::new("-").unwrap();
            let result = lency_string_join(vec, sep.as_ptr());
            assert!(!result.is_null());

            let joined = CStr::from_ptr(result).to_str().unwrap();
            assert_eq!(joined, "hello-world-test");

            // 清理
            libc::free(result as *mut libc::c_void);
            for i in 0..(*vec).len() {
                libc::free((*vec).get(i) as *mut libc::c_void);
            }
            let _ = Box::from_raw(vec);
        }
    }

    #[test]
    fn test_string_substr() {
        let s = CString::new("hello world").unwrap();

        let result = unsafe { lency_string_substr(s.as_ptr(), 0, 5) };
        assert!(!result.is_null());
        let substr = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
        assert_eq!(substr, "hello");
        unsafe { libc::free(result as *mut libc::c_void) };

        let result2 = unsafe { lency_string_substr(s.as_ptr(), 6, 5) };
        let substr2 = unsafe { CStr::from_ptr(result2) }.to_str().unwrap();
        assert_eq!(substr2, "world");
        unsafe { libc::free(result2 as *mut libc::c_void) };
    }
}
