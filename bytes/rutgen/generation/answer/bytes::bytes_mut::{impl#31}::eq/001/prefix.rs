// Answer 0

#[test]
fn test_eq_same_content() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3, 4, 5]))).unwrap(),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let other: &[u8] = &[1, 2, 3, 4, 5];
    bytes_mut.eq(other);
}

#[test]
fn test_eq_different_content() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3, 4, 5]))).unwrap(),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let other: &[u8] = &[5, 4, 3, 2, 1];
    bytes_mut.eq(other);
}

#[test]
fn test_eq_different_length() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3]))).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let other: &[u8] = &[1, 2, 3, 4, 5];
    bytes_mut.eq(other);
}

#[test]
fn test_eq_empty_other() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3, 4, 5]))).unwrap(),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let other: &[u8] = &[];
    bytes_mut.eq(other);
}

#[test]
fn test_eq_empty_self() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([])).unwrap()).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let other: &[u8] = &[];
    bytes_mut.eq(other);
}

