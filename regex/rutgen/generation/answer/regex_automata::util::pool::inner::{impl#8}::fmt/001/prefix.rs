// Answer 0

#[test]
fn test_pool_guard_debug_non_empty_pool_empty_value() {
    struct TestData;
    impl Send for TestData {}
    impl core::fmt::Debug for TestData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "TestData")
        }
    }
    
    let pool_func = || TestData;
    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestData)]),
        create: pool_func,
    };
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };

    let _ = guard.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_pool_guard_debug_empty_pool_none_value() {
    struct TestData;
    impl Send for TestData {}
    impl core::fmt::Debug for TestData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "TestData")
        }
    }
    
    let pool_func = || TestData;
    let pool = Pool {
        stack: Mutex::new(vec![]),
        create: pool_func,
    };
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };

    let _ = guard.fmt(&mut core::fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_pool_guard_debug_invalid_function_type() {
    struct TestData;
    impl Send for TestData {}
    impl core::fmt::Debug for TestData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "TestData")
        }
    }

    let invalid_pool_func = |_: &str| TestData; // Invalid signature
    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestData)]),
        create: invalid_pool_func,
    };
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };

    let _ = guard.fmt(&mut core::fmt::Formatter::new());
}

