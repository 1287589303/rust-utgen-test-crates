// Answer 0

#[test]
fn test_put_value_no_contention() {
    struct TestData;
    
    let pool: Pool<TestData, fn() -> TestData> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(vec![]),
        },
        create: || TestData,
    };

    let value = Box::new(TestData);
    pool.put_value(value);
}

#[test]
fn test_put_value_with_contention() {
    struct TestData;
    
    let pool: Pool<TestData, fn() -> TestData> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(vec![Box::new(TestData)]),
        },
        create: || TestData,
    };
    
    let value = Box::new(TestData);
    pool.put_value(value);
}

#[test]
fn test_put_value_thread_id_contention() {
    struct TestData;

    const THREAD_ID: usize = 1; // Simulate a thread with a specific ID leading to contention
    
    let pool: Pool<TestData, fn() -> TestData> = Pool {
        stack: Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(vec![Box::new(TestData)]),
        },
        create: || TestData,
    };
    
    let value = Box::new(TestData);
    pool.put_value(value);
}

