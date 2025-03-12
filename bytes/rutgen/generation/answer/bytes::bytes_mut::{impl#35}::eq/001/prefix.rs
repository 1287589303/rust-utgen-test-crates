// Answer 0

#[test]
fn test_eq_empty_bytesmut_with_empty_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(ptr::null_mut()),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let other = "";
    bytes_mut.eq(other);
}

#[test]
fn test_eq_non_empty_bytesmut_with_non_matching_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(Box::into_raw(Box::new([104, 101, 108, 108, 111]))),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let other = "world";
    bytes_mut.eq(other);
}

#[test]
fn test_eq_non_empty_bytesmut_with_matching_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(Box::into_raw(Box::new([104, 101, 108, 108, 111]))),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let other = "hello";
    bytes_mut.eq(other);
}

#[test]
fn test_eq_non_empty_bytesmut_with_utf8_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(Box::into_raw(Box::new([231, 164, 186, 231, 164, 186]))),
        len: 6,
        cap: 6,
        data: ptr::null_mut(),
    };
    let other = "你好";
    bytes_mut.eq(other);
}

#[test]
fn test_eq_boundary_case_bytesmut_with_very_long_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new_unchecked(Box::into_raw(Box::new([97; 1000]))), // 1000 bytes of 'a'
        len: 1000,
        cap: 1000,
        data: ptr::null_mut(),
    };
    let other = "a".repeat(1000);
    bytes_mut.eq(other);
}

