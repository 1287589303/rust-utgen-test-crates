// Answer 0

#[test]
fn test_clone_owned_vtable() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([1u8; 10])) as *const u8,
        len: 10,
        data,
        vtable: &OWNED_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_promotable_even_vtable() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([2u8; 20])) as *const u8,
        len: 20,
        data,
        vtable: &PROMOTABLE_EVEN_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_promotable_odd_vtable() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([3u8; 30])) as *const u8,
        len: 30,
        data,
        vtable: &PROMOTABLE_ODD_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_shared_vtable() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([4u8; 40])) as *const u8,
        len: 40,
        data,
        vtable: &SHARED_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_static_vtable() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([5u8; 50])) as *const u8,
        len: 50,
        data,
        vtable: &STATIC_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_zero_length() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: std::ptr::null(),
        len: 0,
        data,
        vtable: &OWNED_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_null_pointer() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(42)) as *mut ());
    let bytes = Bytes {
        ptr: std::ptr::null(),
        len: 10,
        data,
        vtable: &PROMOTABLE_EVEN_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_unique_data() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(21)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([6u8; 60])) as *const u8,
        len: 60,
        data,
        vtable: &SHARED_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

#[test]
fn test_clone_non_unique_data() {
    let data = AtomicPtr::new(Box::into_raw(Box::new(21)) as *mut ());
    let bytes = Bytes {
        ptr: Box::into_raw(Box::new([7u8; 70])) as *const u8,
        len: 70,
        data,
        vtable: &SHARED_VTABLE,
    };
    let _cloned_bytes = bytes.clone();
}

