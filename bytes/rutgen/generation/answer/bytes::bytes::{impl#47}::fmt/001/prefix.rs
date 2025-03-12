// Answer 0

#[test]
fn test_fmt_with_owned_vtable() {
    let ptr: *const u8 = &0u8 as *const u8; // valid non-null pointer
    let len: usize = 1; // non-negative integer
    let vtable: &'static Vtable = &OWNED_VTABLE;

    let bytes = Bytes { ptr, len, data: AtomicPtr::new(ptr as *mut ()), vtable };

    let mut formatter = fmt::Formatter::new();
    let _ = bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_promotable_even_vtable() {
    let ptr: *const u8 = &1u8 as *const u8; // valid non-null pointer
    let len: usize = 2; // non-negative integer
    let vtable: &'static Vtable = &PROMOTABLE_EVEN_VTABLE;

    let bytes = Bytes { ptr, len, data: AtomicPtr::new(ptr as *mut ()), vtable };

    let mut formatter = fmt::Formatter::new();
    let _ = bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_promotable_odd_vtable() {
    let ptr: *const u8 = &2u8 as *const u8; // valid non-null pointer
    let len: usize = 3; // non-negative integer
    let vtable: &'static Vtable = &PROMOTABLE_ODD_VTABLE;

    let bytes = Bytes { ptr, len, data: AtomicPtr::new(ptr as *mut ()), vtable };

    let mut formatter = fmt::Formatter::new();
    let _ = bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_shared_vtable() {
    let ptr: *const u8 = &3u8 as *const u8; // valid non-null pointer
    let len: usize = 4; // non-negative integer
    let vtable: &'static Vtable = &SHARED_VTABLE;

    let bytes = Bytes { ptr, len, data: AtomicPtr::new(ptr as *mut ()), vtable };

    let mut formatter = fmt::Formatter::new();
    let _ = bytes.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_static_vtable() {
    let ptr: *const u8 = &4u8 as *const u8; // valid non-null pointer
    let len: usize = 5; // non-negative integer
    let vtable: &'static Vtable = &STATIC_VTABLE;

    let bytes = Bytes { ptr, len, data: AtomicPtr::new(ptr as *mut ()), vtable };

    let mut formatter = fmt::Formatter::new();
    let _ = bytes.fmt(&mut formatter);
}

