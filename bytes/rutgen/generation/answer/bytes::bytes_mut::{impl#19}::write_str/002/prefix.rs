// Answer 0

#[test]
fn test_write_str_empty_source() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 1,
        data: ptr::null_mut(),
    };
    let result = bytes_mut.write_str("");
}

#[test]
fn test_write_str_one_byte_source() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 1,
        data: ptr::null_mut(),
    };
    let result = bytes_mut.write_str("a");
}

#[test]
fn test_write_str_two_bytes_source_with_insufficient_capacity() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 1,
        cap: 2,
        data: ptr::null_mut(),
    };
    let result = bytes_mut.write_str("ab");
}

#[test]
fn test_write_str_larger_source() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 2,
        cap: 3,
        data: ptr::null_mut(),
    };
    let result = bytes_mut.write_str("abc");
}

#[test]
fn test_write_str_beyond_capacity() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let result = bytes_mut.write_str("abcd");
}

