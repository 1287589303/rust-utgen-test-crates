// Answer 0

#[test]
fn test_once_box_with_valid_pointer() {
    let val = Box::new(42);
    let ptr = Box::into_raw(val);
    let once_box = OnceBox {
        inner: AtomicPtr::new(ptr),
        ghost: PhantomData,
    };
    let _ = core::fmt::Debug::fmt(&once_box, &mut core::fmt::Formatter::new());
}

#[test]
fn test_once_box_with_null_pointer() {
    let once_box = OnceBox {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };
    let _ = core::fmt::Debug::fmt(&once_box, &mut core::fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_once_box_with_deallocated_pointer() {
    let val = Box::new(42);
    let ptr = Box::into_raw(val);
    unsafe {
        // Simulate deallocation
        drop(Box::from_raw(ptr));
    }
    let once_box = OnceBox {
        inner: AtomicPtr::new(ptr),
        ghost: PhantomData,
    };
    let _ = core::fmt::Debug::fmt(&once_box, &mut core::fmt::Formatter::new());
}

