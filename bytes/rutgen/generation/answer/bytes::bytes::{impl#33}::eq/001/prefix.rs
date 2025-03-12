// Answer 0

#[test]
fn test_eq_identical_bytes() {
    let bytes1 = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes1.eq(&bytes2);
}

#[test]
fn test_eq_empty_vs_non_empty_bytes() {
    let bytes1 = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: std::ptr::null(),
        len: 10,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes1.eq(&bytes2);
}

#[test]
fn test_eq_identical_non_empty_bytes() {
    let bytes1 = Bytes {
        ptr: std::ptr::null(),
        len: 10,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: std::ptr::null(),
        len: 10,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes1.eq(&bytes2);
}

#[test]
fn test_eq_different_bytes() {
    let bytes1 = Bytes {
        ptr: std::ptr::null(),
        len: 10,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let bytes2 = Bytes {
        ptr: std::ptr::null(),
        len: 15,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    bytes1.eq(&bytes2);
}

#[test]
fn test_eq_bytes_vs_string() {
    let bytes = Bytes {
        ptr: std::ptr::null(),
        len: 5,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let s = "hello";
    bytes.eq(&s);
}

#[test]
fn test_eq_bytes_vs_vector() {
    let bytes = Bytes {
        ptr: std::ptr::null(),
        len: 5,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &SHARED_VTABLE,
    };
    let vec = vec![104, 101, 108, 108, 111]; // 'hello' in ASCII
    bytes.eq(&vec);
}

