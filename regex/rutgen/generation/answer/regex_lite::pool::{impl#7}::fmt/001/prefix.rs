// Answer 0

#[test]
fn test_fmt_with_some_value() {
    let create_fn: CachePoolFn = Box::new(|| pikevm::Cache::new());
    let pool = Pool {
        stack: Mutex::new(vec![Box::new(pikevm::Cache::new())]),
        create: create_fn,
    };
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(pikevm::Cache::new())),
    };
    let mut formatter = core::fmt::Formatter::default();
    guard.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_none_value() {
    let create_fn: CachePoolFn = Box::new(|| pikevm::Cache::new());
    let pool = Pool {
        stack: Mutex::new(vec![]),
        create: create_fn,
    };
    let guard = PoolGuard {
        pool: &pool,
        value: None,
    };
    let mut formatter = core::fmt::Formatter::default();
    guard.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_empty_pool() {
    let create_fn: CachePoolFn = Box::new(|| pikevm::Cache::new());
    let pool = Pool {
        stack: Mutex::new(vec![]),
        create: create_fn,
    };
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(pikevm::Cache::new())),
    };
    let mut formatter = core::fmt::Formatter::default();
    guard.fmt(&mut formatter);
}

