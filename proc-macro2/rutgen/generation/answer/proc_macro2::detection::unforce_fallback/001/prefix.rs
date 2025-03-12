// Answer 0

#[test]
#[cfg(no_is_available)]
fn test_unforce_fallback_with_successful_panic_catch() {
    unforce_fallback();
}

#[test]
#[cfg(no_is_available)]
fn test_unforce_fallback_with_failed_panic_catch() {
    use std::panic;

    let result = panic::catch_unwind(|| {
        unforce_fallback();
        panic!("expected panic!");
    });

    if result.is_err() {
        unforce_fallback();
    }
}

#[test]
#[cfg(no_is_available)]
fn test_unforce_fallback_with_custom_panic_hook() {
    use std::panic;

    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    unforce_fallback();

    panic::set_hook(original_hook);
}

