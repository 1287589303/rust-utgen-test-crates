// Answer 0

#[test]
fn test_eq_identical_bytes() {
    let data1 = vec![1, 2, 3, 4, 5];
    let data2 = vec![1, 2, 3, 4, 5];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let _ = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_different_length_bytes() {
    let data1 = vec![1, 2, 3];
    let data2 = vec![1, 2, 3, 4, 5];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let _ = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_different_data_same_length() {
    let data1 = vec![1, 2, 3, 4, 5];
    let data2 = vec![5, 4, 3, 2, 1];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let _ = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_empty_bytes() {
    let data1 = vec![];
    let data2 = vec![];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let _ = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_uninitialized_bytes() {
    let bytes1 = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let _ = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_max_u8_bytes() {
    let data1 = vec![255, 255, 255];
    let data2 = vec![255, 255, 255];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let _ = bytes1.eq(&bytes2);
}

