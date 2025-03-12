// Answer 0

#[test]
fn test_put_imp_with_ok_value_and_discard_true() {
    struct Dummy {
        value: i32,
    }

    let create_fn = || Box::new(Dummy { value: 42 });
    let pool = Pool {
        stack: Mutex::new(Vec::new()),
        create: create_fn,
    };
    
    let mut guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(Dummy { value: 42 })),
    };
    
    guard.discard = true;

    guard.put_imp();
}

#[test]
fn test_put_imp_with_ok_value_and_discard_true_edge_case() {
    struct Dummy {
        value: i32,
    }

    let create_fn = || Box::new(Dummy { value: 0 });
    let pool = Pool {
        stack: Mutex::new(Vec::new()),
        create: create_fn,
    };

    let mut guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(Dummy { value: 0 })),
    };

    guard.discard = true;

    guard.put_imp();
}

