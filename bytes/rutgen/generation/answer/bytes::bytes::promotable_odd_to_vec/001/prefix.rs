// Answer 0

#[test]
fn test_promotable_odd_to_vec_zero_length() {
    let data = AtomicPtr::new(NonNull::new(Box::into_raw(Box::new(1u8))).unwrap().as_ptr() as *mut ());
    let ptr: *const u8 = NonNull::new(Box::into_raw(Box::new(0u8))).unwrap().as_ptr();
    let len: usize = 0;

    unsafe { promotable_odd_to_vec(&data, ptr, len) };
}

#[test]
fn test_promotable_odd_to_vec_one_length() {
    let data = AtomicPtr::new(NonNull::new(Box::into_raw(Box::new(1u8))).unwrap().as_ptr() as *mut ());
    let ptr: *const u8 = NonNull::new(Box::into_raw(Box::new(42u8))).unwrap().as_ptr();
    let len: usize = 1;

    unsafe { promotable_odd_to_vec(&data, ptr, len) };
}

#[test]
fn test_promotable_odd_to_vec_max_length() {
    let data = AtomicPtr::new(NonNull::new(Box::into_raw(Box::new(1u8))).unwrap().as_ptr() as *mut ());
    let max_len: usize = usize::MAX;
    let ptr: *const u8 = NonNull::new(Box::into_raw(Box::new([0u8; usize::MAX as usize]))).unwrap().as_ptr();

    unsafe { promotable_odd_to_vec(&data, ptr, max_len) };
}

