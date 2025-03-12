// Answer 0

#[test]
fn test_get_or_init_min_value() {
    let once = OnceNonZeroUsize::new();
    let value = once.get_or_init(|| NonZeroUsize::new(1).unwrap());
    let result = once.get();
}

#[test]
fn test_get_or_init_large_value() {
    let once = OnceNonZeroUsize::new();
    let value = once.get_or_init(|| NonZeroUsize::new(100).unwrap());
    let result = once.get();
}

#[test]
fn test_get_or_init_concurrent() {
    use std::thread;
    let once = OnceNonZeroUsize::new();
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn({
                let once = &once;
                move || once.get_or_init(|| NonZeroUsize::new(2).unwrap())
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.join();
    }

    let result = once.get();
}

#[test]
fn test_get_or_init_repeatable() {
    let once = OnceNonZeroUsize::new();
    let value = once.get_or_init(|| NonZeroUsize::new(3).unwrap());
    let value_again = once.get_or_init(|| NonZeroUsize::new(4).unwrap());
    let result = once.get();
}

#[test]
fn test_get_or_init_empty_initialization() {
    let once = OnceNonZeroUsize::new();
    let value = once.get_or_init(|| NonZeroUsize::new(5).unwrap());
    let result = once.get();
}

