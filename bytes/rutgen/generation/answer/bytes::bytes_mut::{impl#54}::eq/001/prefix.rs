// Answer 0

#[test]
fn test_eq_empty_bytesmut_with_empty_bytes() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([]))).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let bytes = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let _ = bytes_mut.eq(&bytes);
}

#[test]
fn test_eq_non_empty_bytesmut_with_non_empty_bytes() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(data.clone())).cast::<u8>()),
        len: data.len(),
        cap: data.len(),
        data: ptr::null_mut(),
    };
    let bytes = Bytes {
        ptr: data.as_ptr(),
        len: data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let _ = bytes_mut.eq(&bytes);
}

#[test]
fn test_eq_different_length_bytesmut_and_bytes() {
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([1, 2, 3])).cast::<u8>()).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([1, 2, 3, 4])).cast::<u8>(),
        len: 4,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let _ = bytes_mut.eq(&bytes);
}

#[test]
fn test_eq_identical_bytesmut_and_bytes() {
    let data: Vec<u8> = vec![10, 20, 30];
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(data.clone())).cast::<u8>()).unwrap(),
        len: data.len(),
        cap: data.len(),
        data: ptr::null_mut(),
    };
    let bytes = Bytes {
        ptr: data.as_ptr(),
        len: data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let _ = bytes_mut.eq(&bytes);
}

#[test]
fn test_eq_large_bytesmut_with_large_bytes() {
    let data: Vec<u8> = (0..MAX_VEC_POS).map(|x| x as u8).collect();
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(data.clone())).cast::<u8>()).unwrap(),
        len: data.len(),
        cap: data.len(),
        data: ptr::null_mut(),
    };
    let bytes = Bytes {
        ptr: data.as_ptr(),
        len: data.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let _ = bytes_mut.eq(&bytes);
}

