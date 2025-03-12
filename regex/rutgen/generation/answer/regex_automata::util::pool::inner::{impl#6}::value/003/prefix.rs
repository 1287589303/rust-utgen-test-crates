// Answer 0

#[test]
fn test_value_ok() {
    struct TestValue;
    
    let create_fn = || Box::new(TestValue);
    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestValue)]),
        create: create_fn,
    };
    
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(TestValue)),
    };

    let _result = guard.value();
}

#[test]
fn test_value_ok_multiple() {
    struct TestValue;
    
    let create_fn = || Box::new(TestValue);
    let pool = Pool {
        stack: Mutex::new(vec![Box::new(TestValue), Box::new(TestValue)]),
        create: create_fn,
    };
    
    let guard1 = PoolGuard {
        pool: &pool,
        value: Some(Box::new(TestValue)),
    };
    
    let guard2 = PoolGuard {
        pool: &pool,
        value: Some(Box::new(TestValue)),
    };

    let _result1 = guard1.value();
    let _result2 = guard2.value();
}

