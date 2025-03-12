// Answer 0

#[test]
fn test_into_iter_non_empty() {
    let bytes: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1u8, 2, 3, 4, 5]))).unwrap(),
        len: 5,
        cap: 5,
        data: core::ptr::null_mut(),
    };
    let _iter = bytes.into_iter();
}

#[test]
fn test_into_iter_capacity_greater_than_length() {
    let bytes: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1u8, 2, 3, 4, 5, 0, 0]))).unwrap(),
        len: 5,
        cap: 7,
        data: core::ptr::null_mut(),
    };
    let _iter = bytes.into_iter();
}

#[test]
fn test_into_iter_initialization_with_non_null_ptr() {
    let bytes: BytesMut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([10u8, 20, 30]))).unwrap(),
        len: 3,
        cap: 3,
        data: core::ptr::null_mut(),
    };
    let _iter = bytes.into_iter();
}

#[test]
fn test_into_iter_large_length() {
    let data = (0..1024).map(|i| i as u8).collect::<Vec<u8>>();
    let bytes: BytesMut = BytesMut {
        ptr: NonNull::new(data.as_ptr() as *mut u8).unwrap(),
        len: data.len(),
        cap: data.len(),
        data: core::ptr::null_mut(),
    };
    let _iter = bytes.into_iter();
}

