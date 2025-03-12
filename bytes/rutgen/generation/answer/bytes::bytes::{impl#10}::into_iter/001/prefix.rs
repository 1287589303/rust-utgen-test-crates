// Answer 0

#[test]
fn test_into_iter_zero_length() {
    let bytes = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _iter = bytes.into_iter();
}

#[test]
fn test_into_iter_one_length() {
    let data = [0u8; 1];
    let bytes = Bytes {
        ptr: data.as_ptr(),
        len: 1,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _iter = bytes.into_iter();
}

#[test]
fn test_into_iter_max_length() {
    let size = std::usize::MAX;
    let vec = vec![0u8; size];
    let bytes = Bytes {
        ptr: vec.as_ptr(),
        len: size,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _iter = bytes.into_iter();
}

#[test]
#[should_panic]
fn test_into_iter_invalid_pointer() {
    let bytes = Bytes {
        ptr: std::ptr::null_mut(),
        len: 1,
        data: AtomicPtr::new(std::ptr::null_mut()),
        vtable: &OWNED_VTABLE,
    };
    let _iter = bytes.into_iter();
}

