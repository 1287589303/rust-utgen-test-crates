// Answer 0

#[test]
fn test_eq_bytes_mut_with_same_empty_vec() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0][..]))).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let vec_bytes = vec![];
    bytes_mut.eq(&bytes_mut);
}

#[test]
fn test_eq_bytes_mut_with_different_empty_vec() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0][..]))).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let vec_bytes = vec![1u8];
    bytes_mut.eq(&vec_bytes);
}

#[test]
fn test_eq_bytes_mut_with_identical_data() {
    let data = vec![1u8, 2u8, 3u8];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1u8, 2u8, 3u8][..]))).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let vec_bytes = vec![1u8, 2u8, 3u8];
    bytes_mut.eq(&vec_bytes);
}

#[test]
fn test_eq_bytes_mut_with_different_data() {
    let data = vec![1u8, 2u8, 3u8];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1u8, 2u8, 3u8][..]))).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let vec_bytes = vec![3u8, 2u8, 1u8];
    bytes_mut.eq(&vec_bytes);
}

#[test]
fn test_eq_bytes_mut_with_larger_vec() {
    let data = vec![1u8; 1024];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1u8; 1024][..]))).unwrap(),
        len: 1024,
        cap: 1024,
        data: ptr::null_mut(),
    };
    let vec_bytes = vec![1u8; 1024];
    bytes_mut.eq(&vec_bytes);
}

#[test]
fn test_eq_bytes_mut_with_max_capacity() {
    let data = vec![0u8; usize::MAX >> 1]; // Arbitrary size close to max capacity for testing
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(data.clone())).unwrap()),
        len: data.len(),
        cap: data.len(),
        data: ptr::null_mut(),
    };
    let vec_bytes = data;
    bytes_mut.eq(&vec_bytes);
}

