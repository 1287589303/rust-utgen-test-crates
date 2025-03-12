// Answer 0

#[test]
fn test_hash_empty_bytesmut() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0]))).unwrap(),
        len: 0,
        cap: 1,
        data: std::ptr::null_mut(),
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
fn test_hash_non_empty_bytesmut() {
    let vec = vec![1, 2, 3, 4, 5];
    let bytes_mut = BytesMut {
        ptr: NonNull::from(vec.as_ptr() as *mut u8),
        len: vec.len(),
        cap: vec.capacity(),
        data: std::ptr::null_mut(),
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
fn test_hash_large_bytesmut() {
    let vec = vec![0u8; std::usize::MAX / 2];
    let bytes_mut = BytesMut {
        ptr: NonNull::from(vec.as_ptr() as *mut u8),
        len: vec.len(),
        cap: vec.capacity(),
        data: std::ptr::null_mut(),
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes_mut.hash(&mut hasher);
}

#[test]
#[should_panic]
fn test_hash_invalid_capacity_bytesmut() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([0u8; 0]))).unwrap(),
        len: 0,
        cap: 0, // Invalid capacity
        data: std::ptr::null_mut(),
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes_mut.hash(&mut hasher);
}

