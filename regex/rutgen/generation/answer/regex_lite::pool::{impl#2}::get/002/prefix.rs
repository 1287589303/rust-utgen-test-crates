// Answer 0

#[test]
fn test_get_empty_stack_creates_new_value() {
    struct TestCache;
    let create_fn: CachePoolFn = Box::new(|| TestCache {});
    let pool = CachePool {
        stack: Mutex::new(vec![]),
        create: create_fn,
    };
    
    let guard = pool.get();
}

#[test]
fn test_get_concurrent_empty_stack_creates_new_value() {
    use std::thread;

    struct TestCache;
    let create_fn: CachePoolFn = Box::new(|| TestCache {});
    let pool = CachePool {
        stack: Mutex::new(vec![]),
        create: create_fn,
    };
    
    let handles: Vec<_> = (0..10).map(|_| {
        let pool_clone = &pool;
        thread::spawn(move || {
            let guard = pool_clone.get();
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

