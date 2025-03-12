// Answer 0

#[test]
fn test_promotable_even_clone_with_kind_vec() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new(0u8)) as *mut u8,
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut _);
    let valid_ptr: *const u8 = Box::into_raw(Box::new(1u8));
    let len: usize = 1;

    let cloned_bytes = unsafe {
        promotable_even_clone(&atomic_ptr, valid_ptr, len)
    };

    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.ptr, valid_ptr);
}

#[test]
fn test_promotable_even_clone_with_large_len() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 1024])) as *mut u8,
        cap: 1024,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut _);
    let valid_ptr: *const u8 = Box::into_raw(Box::new(2u8));
    let len: usize = 1024;

    let cloned_bytes = unsafe {
        promotable_even_clone(&atomic_ptr, valid_ptr, len)
    };

    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.ptr, valid_ptr);
}

#[test]
fn test_promotable_even_clone_with_min_len() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new(3u8)) as *mut u8,
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut _);
    let valid_ptr: *const u8 = Box::into_raw(Box::new(5u8));
    let len: usize = 1;

    let cloned_bytes = unsafe {
        promotable_even_clone(&atomic_ptr, valid_ptr, len)
    };

    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.ptr, valid_ptr);
}

#[test]
#[should_panic]
fn test_promotable_even_clone_with_invalid_atomic_ptr() {
    let invalid_atomic_ptr = AtomicPtr::new(core::ptr::null_mut());
    let valid_ptr: *const u8 = Box::into_raw(Box::new(6u8));
    let len: usize = 1;

    unsafe {
        promotable_even_clone(&invalid_atomic_ptr, valid_ptr, len);
    }
}

