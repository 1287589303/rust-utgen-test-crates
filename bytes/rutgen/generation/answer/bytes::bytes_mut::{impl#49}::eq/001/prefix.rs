// Answer 0

#[test]
fn test_eq_empty_string_and_bytesmut() {
    let string_slice: &str = "";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 0,
        data: std::ptr::null_mut(),
    };
    let _ = string_slice.eq(&bytes_mut);
}

#[test]
fn test_eq_equal_string_and_bytesmut() {
    let string_slice: &str = "hello";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 5,
        cap: 5,
        data: std::ptr::null_mut(),
    };
    let _ = string_slice.eq(&bytes_mut);
}

#[test]
fn test_eq_different_length_string_and_bytesmut() {
    let string_slice: &str = "hello";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 4,
        cap: 5,
        data: std::ptr::null_mut(),
    };
    let _ = string_slice.eq(&bytes_mut);
}

#[test]
fn test_eq_string_and_bytesmut_with_null_data() {
    let string_slice: &str = "test";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 4,
        cap: 4,
        data: std::ptr::null_mut(),
    };
    let _ = string_slice.eq(&bytes_mut);
}

