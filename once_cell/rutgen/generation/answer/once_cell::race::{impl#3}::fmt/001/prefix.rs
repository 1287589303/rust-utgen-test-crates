// Answer 0

#[test]
fn test_once_ref_debug_with_null_inner() {
    let once_ref = OnceRef::<i32> {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };
    let _ = format!("{:?}", once_ref);
}

#[test]
fn test_once_ref_debug_with_valid_inner() {
    let value = Box::new(42);
    let once_ref = OnceRef::<i32> {
        inner: AtomicPtr::new(Box::into_raw(value)),
        ghost: PhantomData,
    };
    let _ = format!("{:?}", once_ref);
}

#[test]
fn test_once_ref_debug_with_edge_case_inner() {
    let once_ref = OnceRef::<NonZeroUsize> {
        inner: AtomicPtr::new(ptr::null_mut()),
        ghost: PhantomData,
    };
    let _ = format!("{:?}", once_ref);
}

