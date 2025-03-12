// Answer 0

#[test]
fn test_bytes_eq_bytesmut_equal_lengths() {
    let bytes_data = vec![1, 2, 3, 4, 5];
    let bytes = Bytes {
        ptr: bytes_data.as_ptr(),
        len: bytes_data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_mut_data = vec![1, 2, 3, 4, 5];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(bytes_mut_data.as_mut_ptr()).unwrap(),
        len: bytes_mut_data.len(),
        cap: bytes_mut_data.capacity(),
        data: ptr::null_mut(),
    };

    let _ = bytes.eq(&bytes_mut);
}

#[test]
fn test_bytes_eq_bytesmut_different_lengths() {
    let bytes_data = vec![1, 2, 3];
    let bytes = Bytes {
        ptr: bytes_data.as_ptr(),
        len: bytes_data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_mut_data = vec![1, 2, 3, 4, 5];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(bytes_mut_data.as_mut_ptr()).unwrap(),
        len: bytes_mut_data.len(),
        cap: bytes_mut_data.capacity(),
        data: ptr::null_mut(),
    };

    let _ = bytes.eq(&bytes_mut);
}

#[test]
fn test_bytes_eq_bytesmut_empty() {
    let bytes_data: Vec<u8> = Vec::new();
    let bytes = Bytes {
        ptr: bytes_data.as_ptr(),
        len: bytes_data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_mut_data: Vec<u8> = Vec::new();
    let bytes_mut = BytesMut {
        ptr: NonNull::new(bytes_mut_data.as_mut_ptr()).unwrap(),
        len: bytes_mut_data.len(),
        cap: bytes_mut_data.capacity(),
        data: ptr::null_mut(),
    };

    let _ = bytes.eq(&bytes_mut);
}

#[test]
fn test_bytes_eq_bytesmut_single_element_equal() {
    let bytes_data = vec![10];
    let bytes = Bytes {
        ptr: bytes_data.as_ptr(),
        len: bytes_data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_mut_data = vec![10];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(bytes_mut_data.as_mut_ptr()).unwrap(),
        len: bytes_mut_data.len(),
        cap: bytes_mut_data.capacity(),
        data: ptr::null_mut(),
    };

    let _ = bytes.eq(&bytes_mut);
}

#[test]
fn test_bytes_eq_bytesmut_single_element_different() {
    let bytes_data = vec![10];
    let bytes = Bytes {
        ptr: bytes_data.as_ptr(),
        len: bytes_data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_mut_data = vec![20];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(bytes_mut_data.as_mut_ptr()).unwrap(),
        len: bytes_mut_data.len(),
        cap: bytes_mut_data.capacity(),
        data: ptr::null_mut(),
    };

    let _ = bytes.eq(&bytes_mut);
}

#[test]
fn test_bytes_eq_bytesmut_max_length() {
    let max_length_data = vec![1; MAX_VEC_POS]; // Fill with 1s
    let bytes = Bytes {
        ptr: max_length_data.as_ptr(),
        len: max_length_data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_mut = BytesMut {
        ptr: NonNull::new(max_length_data.as_mut_ptr()).unwrap(),
        len: max_length_data.len(),
        cap: max_length_data.capacity(),
        data: ptr::null_mut(),
    };

    let _ = bytes.eq(&bytes_mut);
}

