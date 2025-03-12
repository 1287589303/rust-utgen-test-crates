// Answer 0

#[test]
fn test_partial_cmp_non_empty_str_vs_non_empty_bytes_mut() {
    let non_empty_str: &str = "Hello, World!";
    let bytes_mut: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([70, 101, 108, 108, 111, 33]))).unwrap(),
        len: 6,
        cap: 6,
        data: ptr::null_mut(),
    };
    let _ = non_empty_str.partial_cmp(&bytes_mut);
}

#[test]
fn test_partial_cmp_non_empty_str_vs_max_length_bytes_mut() {
    let non_empty_str: &str = "Hello, World!";
    let bytes_mut: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([65; 1024]))).unwrap(),
        len: 1024,
        cap: 1024,
        data: ptr::null_mut(),
    };
    let _ = non_empty_str.partial_cmp(&bytes_mut);
}

#[test]
fn test_partial_cmp_non_empty_str_vs_min_length_bytes_mut() {
    let non_empty_str: &str = "A";
    let bytes_mut: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([65]))).unwrap(),
        len: 1,
        cap: 1,
        data: ptr::null_mut(),
    };
    let _ = non_empty_str.partial_cmp(&bytes_mut);
}

#[test]
fn test_partial_cmp_different_length_bytes_mut() {
    let non_empty_str: &str = "Test";
    let bytes_mut: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([84, 101, 115, 116, 50]))).unwrap(),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let _ = non_empty_str.partial_cmp(&bytes_mut);
}

#[test]
fn test_partial_cmp_equal_length_bytes_mut() {
    let non_empty_str: &str = "BytesMut";
    let bytes_mut: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([66, 121, 116, 101, 115, 77, 117, 116]))).unwrap(),
        len: 8,
        cap: 8,
        data: ptr::null_mut(),
    };
    let _ = non_empty_str.partial_cmp(&bytes_mut);
}

