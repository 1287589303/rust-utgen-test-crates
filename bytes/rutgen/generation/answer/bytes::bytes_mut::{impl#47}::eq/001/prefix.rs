// Answer 0

#[test]
fn test_eq_non_empty_equal() {
    let bytes_mut = BytesMut {
        ptr: NonNull::from(Box::into_raw(Box::new([1, 2, 3, 4] as [u8; 4]))),
        len: 4,
        cap: 4,
        data: ptr::null_mut(),
    };
    let bytes = Bytes::from_iter(&[1, 2, 3, 4]);
    assert!(bytes_mut.eq(&&bytes));
}

#[test]
fn test_eq_non_empty_unequal() {
    let bytes_mut = BytesMut {
        ptr: NonNull::from(Box::into_raw(Box::new([1, 2, 3, 4] as [u8; 4]))),
        len: 4,
        cap: 4,
        data: ptr::null_mut(),
    };
    let bytes = Bytes::from_iter(&[4, 3, 2, 1]);
    assert!(!bytes_mut.eq(&&bytes));
}

#[test]
fn test_eq_empty() {
    let bytes_mut = BytesMut {
        ptr: NonNull::from(Box::into_raw(Box::new([] as [u8; 0]))),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let bytes = Bytes::from_iter(&[]);
    assert!(bytes_mut.eq(&&bytes));
}

#[test]
fn test_eq_max_length() {
    let max_length: usize = 16; // Assuming a max length for this example
    let bytes_mut = BytesMut {
        ptr: NonNull::from(Box::into_raw(Box::new([0; max_length] as [u8; 16]))),
        len: max_length,
        cap: max_length,
        data: ptr::null_mut(),
    };
    let bytes = Bytes::from_iter(&[0; max_length]);
    assert!(bytes_mut.eq(&&bytes));
}

#[test]
fn test_eq_different_length() {
    let bytes_mut = BytesMut {
        ptr: NonNull::from(Box::into_raw(Box::new([1, 2, 3] as [u8; 3]))),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let bytes = Bytes::from_iter(&[1, 2, 3, 4]);
    assert!(!bytes_mut.eq(&&bytes));
}

