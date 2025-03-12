// Answer 0

#[test]
fn test_pool_guard_debug_with_some_value() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool::<TestType, fn() -> TestType>::new();
    let value = Some(Box::new(TestType));
    let guard = PoolGuard {
        pool: &pool,
        value,
    };
    let _ = core::fmt::format(format_args!("{:?}", guard));
}

#[test]
fn test_pool_guard_debug_with_none_value() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool::<TestType, fn() -> TestType>::new();
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };
    let _ = core::fmt::format(format_args!("{:?}", guard));
}

#[test]
fn test_pool_guard_debug_generic_function() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool::<TestType, fn() -> TestType>::new();
    let value = Some(Box::new(TestType));
    let guard = PoolGuard {
        pool: &pool,
        value,
    };
    let _ = core::fmt::format(format_args!("{:?}", guard));
}

#[test]
fn test_pool_guard_debug_empty_pool() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool::<TestType, fn() -> TestType>::new();
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };
    let _ = core::fmt::format(format_args!("{:?}", guard));
}

