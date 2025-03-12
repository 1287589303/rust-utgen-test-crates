// Answer 0

#[test]
fn test_eq_bytes_equal() {
    let data1: &[u8] = &[1, 2, 3, 4, 5];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    
    let data2: &[u8] = &[1, 2, 3, 4, 5];
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let result = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_bytes_not_equal() {
    let data1: &[u8] = &[1, 2, 3, 4, 5];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let data2: &[u8] = &[5, 4, 3, 2, 1];
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let result = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_bytes_self_empty() {
    let data1: &[u8] = &[];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let data2: &[u8] = &[1, 2, 3, 4, 5];
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let result = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_bytes_other_empty() {
    let data1: &[u8] = &[1, 2, 3, 4, 5];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let data2: &[u8] = &[];
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let result = bytes1.eq(&bytes2);
}

#[test]
fn test_eq_bytes_both_empty() {
    let data1: &[u8] = &[];
    let bytes1 = Bytes {
        ptr: data1.as_ptr(),
        len: data1.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let data2: &[u8] = &[];
    let bytes2 = Bytes {
        ptr: data2.as_ptr(),
        len: data2.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };

    let result = bytes1.eq(&bytes2);
}

