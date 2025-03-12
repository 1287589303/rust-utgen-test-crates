// Answer 0

#[test]
fn test_partial_cmp_equal() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let vec = vec![1, 2, 3];
    bytes_mut.partial_cmp(&vec);
}

#[test]
fn test_partial_cmp_not_equal() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let vec = vec![4, 5, 6];
    bytes_mut.partial_cmp(&vec);
}

#[test]
fn test_partial_cmp_empty_vec() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let vec: Vec<u8> = vec![];
    bytes_mut.partial_cmp(&vec);
}

#[test]
fn test_partial_cmp_boundary_min() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1]))).unwrap(),
        len: 1,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let vec = vec![1];
    bytes_mut.partial_cmp(&vec);
}

#[test]
fn test_partial_cmp_boundary_max() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0; usize::MAX as usize]))).unwrap(),
        len: usize::MAX,
        cap: usize::MAX,
        data: std::ptr::null_mut(),
    };
    let vec = vec![0; usize::MAX as usize];
    bytes_mut.partial_cmp(&vec);
}

