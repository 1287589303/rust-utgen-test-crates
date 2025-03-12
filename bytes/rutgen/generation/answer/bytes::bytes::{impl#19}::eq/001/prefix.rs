// Answer 0

#[test]
fn test_eq_bytes_equal() {
    let data1: Vec<u8> = vec![1, 2, 3];
    let data2: Vec<u8> = vec![1, 2, 3];

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
fn test_eq_bytes_empty_vs_non_empty() {
    let data: Vec<u8> = vec![1, 2, 3];

    let bytes1 = Bytes {
        ptr: data.as_ptr(),
        len: data.len(),
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
fn test_eq_bytes_different_data_same_length() {
    let data1: Vec<u8> = vec![1, 2, 3];
    let data2: Vec<u8> = vec![4, 5, 6];

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
fn test_eq_bytes_empty() {
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

