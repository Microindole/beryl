//! Lency enum runtime support used by self-host LIR lowering.

#[derive(Debug)]
struct LencyEnumValue {
    tag: i64,
    payloads: Vec<i64>,
}

impl LencyEnumValue {
    fn new(tag: i64, payloads: Vec<i64>) -> Box<Self> {
        Box::new(Self { tag, payloads })
    }
}

#[no_mangle]
pub extern "C" fn lency_enum_new0(tag: i64) -> i64 {
    Box::into_raw(LencyEnumValue::new(tag, vec![])) as i64
}

#[no_mangle]
pub extern "C" fn lency_enum_new1(tag: i64, payload0: i64) -> i64 {
    Box::into_raw(LencyEnumValue::new(tag, vec![payload0])) as i64
}

#[no_mangle]
pub extern "C" fn lency_enum_new2(tag: i64, payload0: i64, payload1: i64) -> i64 {
    Box::into_raw(LencyEnumValue::new(tag, vec![payload0, payload1])) as i64
}

#[no_mangle]
pub extern "C" fn lency_enum_new3(tag: i64, payload0: i64, payload1: i64, payload2: i64) -> i64 {
    Box::into_raw(LencyEnumValue::new(tag, vec![payload0, payload1, payload2])) as i64
}

#[no_mangle]
/// Returns the enum tag stored in a runtime enum handle.
///
/// # Safety
/// `handle` must be either `0` or a pointer previously returned by
/// `lency_enum_new0`, `lency_enum_new1`, `lency_enum_new2`, or `lency_enum_new3`.
pub unsafe extern "C" fn lency_enum_tag(handle: i64) -> i64 {
    if handle == 0 {
        return -1;
    }
    let value = unsafe { &*(handle as *const LencyEnumValue) };
    value.tag
}

#[no_mangle]
/// Returns the payload value at `index` from a runtime enum handle.
///
/// # Safety
/// `handle` must be either `0` or a pointer previously returned by
/// `lency_enum_new0`, `lency_enum_new1`, `lency_enum_new2`, or `lency_enum_new3`.
pub unsafe extern "C" fn lency_enum_payload(handle: i64, index: i64) -> i64 {
    if handle == 0 || index < 0 {
        return 0;
    }
    let value = unsafe { &*(handle as *const LencyEnumValue) };
    value.payloads.get(index as usize).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_new0_and_tag() {
        let handle = lency_enum_new0(3);
        let tag = unsafe { lency_enum_tag(handle) };
        assert_eq!(tag, 3);
    }

    #[test]
    fn test_enum_payload_roundtrip() {
        let handle = lency_enum_new2(7, 11, 13);
        let first = unsafe { lency_enum_payload(handle, 0) };
        let second = unsafe { lency_enum_payload(handle, 1) };
        let missing = unsafe { lency_enum_payload(handle, 5) };
        assert_eq!(first, 11);
        assert_eq!(second, 13);
        assert_eq!(missing, 0);
    }

    #[test]
    fn test_enum_payload_roundtrip_three_values() {
        let handle = lency_enum_new3(9, 2, 4, 6);
        let first = unsafe { lency_enum_payload(handle, 0) };
        let second = unsafe { lency_enum_payload(handle, 1) };
        let third = unsafe { lency_enum_payload(handle, 2) };
        assert_eq!(unsafe { lency_enum_tag(handle) }, 9);
        assert_eq!(first, 2);
        assert_eq!(second, 4);
        assert_eq!(third, 6);
    }
}
