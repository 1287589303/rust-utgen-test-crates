// Answer 0

#[test]
fn test_eq_identical_non_empty() {
    let bytes_a = Bytes {
        ptr: NonNull::from(&b"hello"[..]).as_ptr(),
        len: 5,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: NonNull::from(&b"hello"[..]).as_ptr(),
        len: 5,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _ = bytes_a.eq(&&bytes_b);
}

#[test]
fn test_eq_identical_empty() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _ = bytes_a.eq(&&bytes_b);
}

#[test]
fn test_eq_non_identical_non_empty() {
    let bytes_a = Bytes {
        ptr: NonNull::from(&b"hello"[..]).as_ptr(),
        len: 5,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: NonNull::from(&b"world"[..]).as_ptr(),
        len: 5,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _ = bytes_a.eq(&&bytes_b);
}

#[test]
fn test_eq_one_empty_one_non_empty() {
    let bytes_a = Bytes {
        ptr: NonNull::from(&b""[..]).as_ptr(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: NonNull::from(&b"hello"[..]).as_ptr(),
        len: 5,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _ = bytes_a.eq(&&bytes_b);
}

#[test]
fn test_eq_both_empty() {
    let bytes_a = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let bytes_b = Bytes {
        ptr: ptr::null(),
        len: 0,
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _ = bytes_a.eq(&&bytes_b);
}

