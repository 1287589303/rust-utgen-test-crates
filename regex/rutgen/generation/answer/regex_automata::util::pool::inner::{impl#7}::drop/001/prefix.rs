// Answer 0

#[test]
fn test_drop_with_active_value() {
    struct TestStruct;

    let create_fn = || TestStruct;
    let pool = Pool { 
        stack: Mutex::new(vec![]), 
        create: create_fn 
    };
    let mut guard = PoolGuard { 
        pool: &pool, 
        value: Some(Box::new(TestStruct)) 
    };
    
    guard.drop();
}

#[test]
fn test_drop_with_none_value() {
    struct TestStruct;

    let create_fn = || TestStruct;
    let pool = Pool { 
        stack: Mutex::new(vec![]), 
        create: create_fn 
    };
    let mut guard = PoolGuard { 
        pool: &pool, 
        value: None 
    };
    
    guard.drop();
}

#[test]
fn test_drop_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    struct TestStruct;

    let create_fn = || TestStruct;
    let pool = Arc::new(Pool { 
        stack: Mutex::new(vec![]), 
        create: create_fn 
    });
    let guard = PoolGuard { 
        pool: &pool, 
        value: Some(Box::new(TestStruct)) 
    };

    let pool_clone = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        let mut guard_clone = guard;
        guard_clone.drop();
    });

    guard.drop();
    handle.join().unwrap();
}

