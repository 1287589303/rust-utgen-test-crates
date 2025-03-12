// Answer 0

#[test]
fn test_shared_to_vec_valid_input() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    let ptr: *const u8 = shared as *const Shared as *const u8;
    let len = 10;

    let data = AtomicPtr::new(shared);
    
    let _result = unsafe { shared_to_vec(&data, ptr, len) };
}

#[test]
fn test_shared_to_vec_zero_length() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    let ptr: *const u8 = shared as *const Shared as *const u8;
    let len = 0;

    let data = AtomicPtr::new(shared);

    let result = unsafe { shared_to_vec(&data, ptr, len) };
    assert!(result.is_empty());
}

#[test]
fn test_shared_to_vec_exceeding_capacity() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 5])) as *mut u8,
        cap: 5,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    let ptr: *const u8 = shared as *const Shared as *const u8;
    let len = 10;

    let data = AtomicPtr::new(shared);

    let _result = unsafe { shared_to_vec(&data, ptr, len) };
}

#[test]
fn test_shared_to_vec_ref_count_non_unique() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(2),
    }));
    
    let ptr: *const u8 = shared as *const Shared as *const u8;
    let len = 10;

    let data = AtomicPtr::new(shared);

    let _result = unsafe { shared_to_vec(&data, ptr, len) };
}

