fn guard_owned(&self, caller: usize) -> PoolGuard<'_, T, F> {
            PoolGuard { pool: self, value: Err(caller), discard: false }
        }