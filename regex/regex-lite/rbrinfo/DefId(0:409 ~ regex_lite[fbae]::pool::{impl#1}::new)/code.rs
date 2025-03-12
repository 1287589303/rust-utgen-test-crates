pub(crate) const fn new(create: F) -> Pool<T, F> {
        Pool { stack: Mutex::new(vec![]), create }
    }