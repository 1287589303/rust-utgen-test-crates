// Answer 0

#[test]
fn test_get_unchecked_valid() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(1).unwrap();
    once.set(value).unwrap();

    unsafe {
        let _result = once.get_unchecked();
    }
}

#[test]
#[should_panic]
fn test_get_unchecked_uninitialized() {
    let once = OnceNonZeroUsize::new();

    unsafe {
        let _result = once.get_unchecked();
    }
}

#[test]
fn test_get_unchecked_boundary_value() {
    use core::num::NonZeroUsize;

    let once = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(usize::MAX).unwrap();
    once.set(value).unwrap();

    unsafe {
        let _result = once.get_unchecked();
    }
}

