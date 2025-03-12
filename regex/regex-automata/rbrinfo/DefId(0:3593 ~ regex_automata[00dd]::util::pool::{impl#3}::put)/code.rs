pub fn put(this: PoolGuard<'_, T, F>) {
        inner::PoolGuard::put(this.0);
    }