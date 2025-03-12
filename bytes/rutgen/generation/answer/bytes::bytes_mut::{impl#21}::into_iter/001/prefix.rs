// Answer 0

#[test]
fn test_into_iter_empty() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0]))).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let _iter = bytes_mut.into_iter();
}

#[test]
fn test_into_iter_small() {
    let vec = vec![1, 2, 3];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(vec.into_boxed_slice()) as *mut u8).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let _iter = bytes_mut.into_iter();
}

#[test]
fn test_into_iter_boundary_max_vec_pos() {
    let vec = vec![0; MAX_VEC_POS];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(vec.into_boxed_slice()) as *mut u8).unwrap(),
        len: MAX_VEC_POS,
        cap: MAX_VEC_POS,
        data: ptr::null_mut(),
    };
    let _iter = bytes_mut.into_iter();
}

#[test]
fn test_into_iter_large_size() {
    let vec = vec![255; 100];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(vec.into_boxed_slice()) as *mut u8).unwrap(),
        len: 100,
        cap: 100,
        data: ptr::null_mut(),
    };
    let _iter = bytes_mut.into_iter();
}

