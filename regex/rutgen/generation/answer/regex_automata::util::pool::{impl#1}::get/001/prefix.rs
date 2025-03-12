// Answer 0

#[test]
fn test_get_with_non_empty_pool() {
    struct TestType;
    let create_fn = || TestType;
    let pool = Pool::<TestType, _>::new(create_fn);
    pool.stack.lock().unwrap().push(Box::new(TestType));
    let guard = pool.get();
}

#[test]
fn test_get_with_empty_pool() {
    struct TestType;
    let create_fn = || TestType;
    let pool = Pool::<TestType, _>::new(create_fn);
    let guard = pool.get();
}

#[test]
fn test_get_concurrent_access() {
    use std::thread;
    struct TestType;
    let create_fn = || TestType;
    let pool = Pool::<TestType, _>::new(create_fn);
    
    pool.stack.lock().unwrap().push(Box::new(TestType));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let pool_clone = &pool;
            thread::spawn(move || {
                let guard = pool_clone.get();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

