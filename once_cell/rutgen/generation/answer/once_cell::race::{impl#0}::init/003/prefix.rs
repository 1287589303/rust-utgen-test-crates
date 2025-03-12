// Answer 0

#[test]
fn test_init_with_nonzero_success() {
    struct TestOnce {
        once: OnceNonZeroUsize,
    }
    
    impl TestOnce {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }
    
        fn init_value(&self) -> NonZeroUsize {
            NonZeroUsize::new(1).unwrap() // returns a NonZeroUsize greater than 0
        }
    }

    let test_instance = TestOnce::new();
    let _ = test_instance.once.init(|_| {
        Ok(test_instance.init_value())
    });
}

#[test]
fn test_init_with_conflict() {
    struct TestOnce {
        once: OnceNonZeroUsize,
    }

    impl TestOnce {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }

        fn other_value(&self) -> Result<NonZeroUsize, ()> {
            Ok(NonZeroUsize::new(2).unwrap()) // returns a different NonZeroUsize greater than 0
        }
    }

    let test_instance = TestOnce::new();
    let _ = test_instance.once.init(|_| {
        let other = test_instance.other_value();
        match other {
            Ok(val) => Ok(val),
            Err(_) => Err(()),
        }
    });
}

#[test]
fn test_init_with_multiple_initializations() {
    struct TestOnce {
        once: OnceNonZeroUsize,
    }

    impl TestOnce {
        fn new() -> Self {
            Self {
                once: OnceNonZeroUsize::new(),
            }
        }

        fn init_value(&self) -> Result<NonZeroUsize, ()> {
            Ok(NonZeroUsize::new(3).unwrap()) // returns a NonZeroUsize greater than 0
        }
    }

    let test_instance = TestOnce::new();
    let _ = test_instance.once.init(|_| {
        test_instance.init_value()
    });
    let _ = test_instance.once.init(|_| {
        test_instance.init_value()
    }); // Should trigger exchange case
}

