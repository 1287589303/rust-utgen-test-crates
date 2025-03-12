// Answer 0

#[test]
fn test_guard_stack_valid_box() {
    struct TestType;
    impl Send for TestType {}

    let create_fn = || TestType;
    let pool = Pool {
        stack: Mutex::new(vec![]),
        create: create_fn,
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack(value);
}

#[test]
fn test_guard_stack_non_null_value() {
    struct TestType;
    impl Send for TestType {}

    let create_fn = || TestType;
    let pool = Pool {
        stack: Mutex::new(vec![]),
        create: create_fn,
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack(value);
}

#[test]
fn test_guard_stack_with_multiple_values() {
    struct TestType;
    impl Send for TestType {}

    let create_fn = || TestType;
    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestType), Box::new(TestType)]),
        create: create_fn,
    };

    let value = Box::new(TestType);
    let guard = pool.guard_stack(value);
}

