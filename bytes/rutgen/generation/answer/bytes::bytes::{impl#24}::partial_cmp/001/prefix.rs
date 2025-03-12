// Answer 0

#[test]
fn test_partial_cmp_non_empty_same_length_equal() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes_a.partial_cmp(&bytes_b);
}

#[test]
fn test_partial_cmp_non_empty_same_length_greater() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes_a.partial_cmp(&bytes_b);
}

#[test]
fn test_partial_cmp_non_empty_different_length() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 5,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes_a.partial_cmp(&bytes_b);
}

#[test]
fn test_partial_cmp_empty_same_length() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes_a.partial_cmp(&bytes_b);
}

#[test]
fn test_partial_cmp_empty_a() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes_a.partial_cmp(&bytes_b);
}

#[test]
fn test_partial_cmp_empty_b() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 3,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes_a.partial_cmp(&bytes_b);
}

