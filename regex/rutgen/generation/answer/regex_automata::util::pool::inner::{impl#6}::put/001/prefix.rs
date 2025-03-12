// Answer 0

#[test]
fn test_put_with_some_value() {
    struct TestValue {
        data: i32,
    }
    
    impl Send for TestValue {}
    
    fn create_value() -> TestValue {
        TestValue { data: 42 }
    }
    
    let pool = Pool {
        stack: Mutex::new(Vec::new()),
        create: create_value,
    };
    
    let value = Some(Box::new(TestValue { data: 1 }));
    let guard = PoolGuard {
        pool: &pool,
        value,
    };
    
    PoolGuard::put(guard);
}

#[test]
fn test_put_with_none_value() {
    struct TestValue {
        data: i32,
    }
    
    impl Send for TestValue {}
    
    fn create_value() -> TestValue {
        TestValue { data: 42 }
    }
    
    let pool = Pool {
        stack: Mutex::new(Vec::new()),
        create: create_value,
    };
    
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };
    
    PoolGuard::put(guard);
}

