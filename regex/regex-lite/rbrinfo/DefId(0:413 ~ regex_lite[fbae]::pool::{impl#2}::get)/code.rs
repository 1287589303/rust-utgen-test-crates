pub(crate) fn get(&self) -> PoolGuard<'_, T, F> {
        let mut stack = self.stack.lock().unwrap();
        let value = match stack.pop() {
            None => Box::new((self.create)()),
            Some(value) => value,
        };
        PoolGuard { pool: self, value: Some(value) }
    }