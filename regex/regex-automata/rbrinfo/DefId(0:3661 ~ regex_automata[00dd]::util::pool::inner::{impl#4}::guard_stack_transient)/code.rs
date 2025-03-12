fn guard_stack_transient(&self, value: Box<T>) -> PoolGuard<'_, T, F> {
            PoolGuard { pool: self, value: Ok(value), discard: true }
        }