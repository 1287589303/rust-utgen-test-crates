// Answer 0

#[test]
fn test_drop_with_null_pointer() {
    use std::sync::atomic::AtomicPtr;
    use std::ptr::null_mut;

    let null_ptr: AtomicPtr<i32> = AtomicPtr::new(null_mut());
    let once_box = OnceBox {
        inner: null_ptr,
        ghost: PhantomData,
    };

    // Calling drop method
    drop(once_box);
}

