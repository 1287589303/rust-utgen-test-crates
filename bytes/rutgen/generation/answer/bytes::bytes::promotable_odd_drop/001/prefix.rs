// Answer 0

#[test]
fn test_promotable_odd_drop_kind_arc() {
    let data = AtomicPtr::new(NonNull::new(Box::into_raw(Box::new(0u8))) as *mut ());
    let ptr = NonNull::new(Box::into_raw(Box::new([0u8; 1]))).unwrap().as_ptr();
    let len = 1;

    unsafe {
        promotable_odd_drop(&mut data, ptr, len);
    }
}

#[test]
fn test_promotable_odd_drop_kind_vec() {
    let data = AtomicPtr::new(NonNull::new(Box::into_raw(Box::new(1u8))) as *mut ());
    let ptr = NonNull::new(Box::into_raw(Box::new([1u8; 2]))).unwrap().as_ptr();
    let len = 2;

    unsafe {
        promotable_odd_drop(&mut data, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_promotable_odd_drop_with_zero_length() {
    let data = AtomicPtr::new(NonNull::new(Box::into_raw(Box::new(0u8))) as *mut ());
    let ptr = NonNull::new(Box::into_raw(Box::new([0u8; 1]))).unwrap().as_ptr();
    let len = 0;

    unsafe {
        promotable_odd_drop(&mut data, ptr, len);
    }
}

