// Answer 0

#[test]
fn test_initialize_available() {
    struct MockProcMacro;

    impl MockProcMacro {
        fn is_available() -> bool {
            true
        }
    }

    let available = MockProcMacro::is_available();
    WORKS.store(available as usize + 1, Ordering::Relaxed);
}

#[test]
fn test_initialize_unavailable() {
    struct MockProcMacro;

    impl MockProcMacro {
        fn is_available() -> bool {
            false
        }
    }

    let available = MockProcMacro::is_available();
    WORKS.store(available as usize + 1, Ordering::Relaxed);
}

#[test]
fn test_initialize_concurrent() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                struct MockProcMacro;

                impl MockProcMacro {
                    fn is_available() -> bool {
                        true
                    }
                }

                let available = MockProcMacro::is_available();
                WORKS.store(available as usize + 1, Ordering::Relaxed);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
} 

#[test]
fn test_initialize_concurrent_unavailable() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                struct MockProcMacro;

                impl MockProcMacro {
                    fn is_available() -> bool {
                        false
                    }
                }

                let available = MockProcMacro::is_available();
                WORKS.store(available as usize + 1, Ordering::Relaxed);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

