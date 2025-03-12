// Answer 0

#[test]
fn test_get_or_try_init_success() {
    let once = OnceNonZeroUsize::new();
    let result = once.get_or_try_init(|| {
        Ok(NonZeroUsize::new(1).unwrap())
    });
    let _ = result; // Consume the result
}

#[test]
fn test_get_or_try_init_failure() {
    let once = OnceNonZeroUsize::new();
    let result = once.get_or_try_init(|| {
        Err("error")
    });
    let _ = result; // Consume the result
}

