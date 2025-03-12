// Answer 0

#[test]
fn test_partial_cmp_equal_lengths() {
    let data = vec![1, 2, 3];
    let bytes_self = Bytes {
        ptr: data.as_ptr(),
        len: data.len(),
        data: AtomicPtr::new(Box::into_raw(Box::new(data))),
        vtable: &OWNED_VTABLE,
    };
    let data_other = vec![1, 2, 3];
    let bytes_other = Bytes {
        ptr: data_other.as_ptr(),
        len: data_other.len(),
        data: AtomicPtr::new(Box::into_raw(Box::new(data_other))),
        vtable: &OWNED_VTABLE,
    };

    bytes_self.partial_cmp(&&bytes_other);
}

#[test]
fn test_partial_cmp_different_lengths() {
    let data_short = vec![1, 2];
    let bytes_short = Bytes {
        ptr: data_short.as_ptr(),
        len: data_short.len(),
        data: AtomicPtr::new(Box::into_raw(Box::new(data_short))),
        vtable: &OWNED_VTABLE,
    };
    let data_long = vec![1, 2, 3];
    let bytes_long = Bytes {
        ptr: data_long.as_ptr(),
        len: data_long.len(),
        data: AtomicPtr::new(Box::into_raw(Box::new(data_long))),
        vtable: &OWNED_VTABLE,
    };

    bytes_short.partial_cmp(&&bytes_long);
}

#[test]
fn test_partial_cmp_empty_bytes() {
    let bytes_empty = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let data_other = vec![1, 2, 3];
    let bytes_other = Bytes {
        ptr: data_other.as_ptr(),
        len: data_other.len(),
        data: AtomicPtr::new(Box::into_raw(Box::new(data_other))),
        vtable: &OWNED_VTABLE,
    };

    bytes_empty.partial_cmp(&&bytes_other);
}

#[test]
fn test_partial_cmp_self_empty_other_empty() {
    let bytes_empty_self = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let bytes_empty_other = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };

    bytes_empty_self.partial_cmp(&&bytes_empty_other);
}

#[test]
fn test_partial_cmp_null_self() {
    let bytes_other = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };

    // Self is null
    let bytes_self = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };

    bytes_self.partial_cmp(&&bytes_other);
}

