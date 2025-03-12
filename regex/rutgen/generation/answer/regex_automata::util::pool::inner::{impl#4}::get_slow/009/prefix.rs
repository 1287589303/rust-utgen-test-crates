// Answer 0

#[test]
fn test_get_slow_owner_in_use() {
    struct TestPool {
        owner: AtomicUsize,
        stacks: Vec<(Mutex<Vec<Box<i32>>>,)>,
    }

    let create_fn = || Box::new(42i32);
    let pool = TestPool {
        owner: AtomicUsize::new(THREAD_ID_INUSE),
        stacks: vec![(Mutex { locked: AtomicBool::new(false), data: UnsafeCell::new(vec![]) })],
    };

    pool.get_slow(1, THREAD_ID_INUSE);
}

#[test]
fn test_get_slow_no_values_in_stack() {
    struct TestPool {
        owner: AtomicUsize,
        stacks: Vec<(Mutex<Vec<Box<i32>>>,)>,
    }

    let create_fn = || Box::new(42i32);
    let pool = TestPool {
        owner: AtomicUsize::new(THREAD_ID_UNOWNED),
        stacks: vec![(Mutex { locked: AtomicBool::new(false), data: UnsafeCell::new(vec![]) })],
    };

    pool.get_slow(1, THREAD_ID_INUSE);
}

