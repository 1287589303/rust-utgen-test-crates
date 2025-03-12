// Answer 0

#[test]
fn test_partial_cmp_equal_lengths() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3, 4]))).unwrap(),
        len: 4,
        cap: 4,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[1, 2, 3, 4];
    let _ = bytes_mut.partial_cmp(other);
}

#[test]
fn test_partial_cmp_varying_lengths() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([5, 6, 7]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[5, 6, 7, 8, 9];
    let _ = bytes_mut.partial_cmp(other);
}

#[test]
fn test_partial_cmp_edge_case_empty() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([10, 11]))).unwrap(),
        len: 2,
        cap: 2,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[];
    let _ = bytes_mut.partial_cmp(other);
}

#[test]
fn test_partial_cmp_edge_case_other_empty() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([12, 13, 14]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[];
    let _ = bytes_mut.partial_cmp(other);
}

#[test]
fn test_partial_cmp_both_equal() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([15, 16, 17, 18]))).unwrap(),
        len: 4,
        cap: 4,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[15, 16, 17, 18];
    let _ = bytes_mut.partial_cmp(other);
}

#[test]
fn test_partial_cmp_first_smaller() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[2, 3, 4];
    let _ = bytes_mut.partial_cmp(other);
}

#[test]
fn test_partial_cmp_first_larger() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([4, 5, 6]))).unwrap(),
        len: 3,
        cap: 3,
        data: std::ptr::null_mut(),
    };
    let other: &[u8] = &[1, 2, 3];
    let _ = bytes_mut.partial_cmp(other);
}

