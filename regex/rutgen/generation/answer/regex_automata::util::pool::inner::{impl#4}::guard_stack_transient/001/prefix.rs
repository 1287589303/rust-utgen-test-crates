// Answer 0

#[test]
fn test_guard_stack_transient_with_valid_value() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestType)]),
        create: || Box::new(TestType),
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack_transient(value);
}

#[test]
fn test_guard_stack_transient_with_empty_stack() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool {
        stack: Mutex::new(vec![]),
        create: || Box::new(TestType),
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack_transient(value);
}

#[test]
fn test_guard_stack_transient_with_boundary_value_at_max_pool_stacks() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool {
        stack: Mutex::new((0..MAX_POOL_STACKS).map(|_| Box::new(TestType)).collect()),
        create: || Box::new(TestType),
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack_transient(value);
}

#[test]
fn test_guard_stack_transient_with_boundary_value_at_one() {
    struct TestType;
    impl Send for TestType {}

    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestType)]),
        create: || Box::new(TestType),
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack_transient(value);
}

