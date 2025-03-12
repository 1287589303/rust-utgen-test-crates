// Answer 0

#[test]
fn test_static_is_unique_non_null() {
    let ptr: NonNull<()> = NonNull::new(Box::into_raw(Box::new(()))).unwrap();
    let atomic_ptr = AtomicPtr::new(ptr.as_ptr());
    static_is_unique(&atomic_ptr);
}

#[test]
fn test_static_is_unique_null() {
    let atomic_ptr = AtomicPtr::new(std::ptr::null_mut());
    static_is_unique(&atomic_ptr);
}

