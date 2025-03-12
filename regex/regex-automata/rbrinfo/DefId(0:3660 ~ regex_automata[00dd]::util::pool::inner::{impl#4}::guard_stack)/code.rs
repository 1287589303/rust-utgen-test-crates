fn guard_stack(&self, value: Box<T>) -> PoolGuard<'_, T, F> {
            PoolGuard { pool: self, value: Ok(value), discard: false }
        }