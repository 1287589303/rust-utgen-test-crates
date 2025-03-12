// Answer 0

#[test]
fn test_with_vtable_static() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let bytes = b"Hello, world!" as *const u8;
    let len = 13;
    let result = unsafe { Bytes::with_vtable(bytes, len, data, &STATIC_VTABLE) };
}

#[test]
fn test_with_vtable_owned() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let bytes = b"Owned Data" as *const u8;
    let len = 10;
    let result = unsafe { Bytes::with_vtable(bytes, len, data, &OWNED_VTABLE) };
}

#[test]
fn test_with_vtable_promotable_even() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let bytes = b"Promotable Even" as *const u8;
    let len = 15;
    let result = unsafe { Bytes::with_vtable(bytes, len, data, &PROMOTABLE_EVEN_VTABLE) };
}

#[test]
fn test_with_vtable_promotable_odd() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let bytes = b"Promotable Odd!" as *const u8;
    let len = 16;
    let result = unsafe { Bytes::with_vtable(bytes, len, data, &PROMOTABLE_ODD_VTABLE) };
}

#[test]
fn test_with_vtable_shared() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let bytes = b"Shared Data!" as *const u8;
    let len = 12;
    let result = unsafe { Bytes::with_vtable(bytes, len, data, &SHARED_VTABLE) };
}

