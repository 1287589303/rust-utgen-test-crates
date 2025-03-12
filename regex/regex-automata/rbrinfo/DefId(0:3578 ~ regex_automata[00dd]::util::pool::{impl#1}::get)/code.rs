pub fn get(&self) -> PoolGuard<'_, T, F> {
        PoolGuard(self.0.get())
    }