fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PoolGuard")
            .field("pool", &self.pool)
            .field("value", &self.value)
            .finish()
    }