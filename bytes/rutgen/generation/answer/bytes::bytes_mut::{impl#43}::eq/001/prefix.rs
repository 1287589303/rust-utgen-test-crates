// Answer 0

#[test]
fn test_eq_empty_bytesmut_with_empty_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 1]))).unwrap(),
        len: 0,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let string = String::new();
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_non_empty_bytesmut_with_empty_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([b'a'; 1]))).unwrap(),
        len: 1,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let string = String::new();
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_empty_bytesmut_with_non_empty_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([])).cast()).unwrap(),
        len: 0,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let string = String::from("a");
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_matching_bytesmut_and_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([b'a'; 1]))).unwrap(),
        len: 1,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let string = String::from("a");
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_non_matching_bytesmut_and_string() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([b'a'; 1]))).unwrap(),
        len: 1,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let string = String::from("b");
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_full_capacity_bytesmut_with_empty_string() {
    let bytes = vec![b'a'; 0x11];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(bytes.into_boxed_slice()).cast()).unwrap(),
        len: 0x11,
        cap: 0x11,
        data: std::ptr::null_mut(),
    };
    let string = String::new();
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_full_capacity_bytesmut_with_matching_string() {
    let bytes = vec![b'a'; 0x11];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(bytes.into_boxed_slice()).cast()).unwrap(),
        len: 0x11,
        cap: 0x11,
        data: std::ptr::null_mut(),
    };
    let string = String::from_iter(vec!['a'; 0x11]);
    let _ = bytes_mut.eq(&string);
}

#[test]
fn test_eq_full_capacity_bytesmut_with_non_matching_string() {
    let bytes = vec![b'a'; 0x11];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(bytes.into_boxed_slice()).cast()).unwrap(),
        len: 0x11,
        cap: 0x11,
        data: std::ptr::null_mut(),
    };
    let string = String::from("b");
    let _ = bytes_mut.eq(&string);
}

