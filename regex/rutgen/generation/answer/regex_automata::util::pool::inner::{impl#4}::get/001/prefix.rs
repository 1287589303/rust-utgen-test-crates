// Answer 0


#[test]
fn test_get_pool_owner() {
    struct TestType;

    let create_fn = || TestType;
    let pool: Pool<TestType, _> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(Vec::new()),
        },
        create: create_fn,
    };

    // Simulate setting the owner to the current thread's identifier.
    let _owner_thread_id = {
        THREAD_ID.with(|id| {
            *id = 1; // Assume 1 is a valid thread ID for this test
        });
        1
    };
    
    let guard = pool.get();
}

#[test]
fn test_get_pool_owner_with_multiple_calls() {
    struct TestType;

    let create_fn = || TestType;
    let pool: Pool<TestType, _> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(Vec::new()),
        },
        create: create_fn,
    };

    // Simulate setting the owner to the current thread's identifier.
    let _owner_thread_id = {
        THREAD_ID.with(|id| {
            *id = 2; // Assume 2 is a valid thread ID for this test
        });
        2
    };
    
    let guard1 = pool.get();
    let guard2 = pool.get();
}

#[test]
fn test_get_pool_owner_empty_stack() {
    struct TestType;

    let create_fn = || TestType;
    let pool: Pool<TestType, _> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(Vec::new()),
        },
        create: create_fn,
    };

    // Simulate setting the owner to the current thread's identifier.
    let _owner_thread_id = {
        THREAD_ID.with(|id| {
            *id = 3; // Assume 3 is a valid thread ID for this test
        });
        3
    };
    
    let guard = pool.get();
}

#[test]
fn test_get_pool_owner_with_different_ids() {
    struct TestType;

    let create_fn = || TestType;
    let pool: Pool<TestType, _> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(Vec::new()),
        },
        create: create_fn,
    };

    // Simulate setting the owner to the current thread's identifier.
    let _owner_thread_id = {
        THREAD_ID.with(|id| {
            *id = 4; // Assume 4 is a valid thread ID for this test
        });
        4
    };

    let guard = pool.get();
    // Note: This test does not invoke another thread to ensure the owner == caller condition remains valid.
}


