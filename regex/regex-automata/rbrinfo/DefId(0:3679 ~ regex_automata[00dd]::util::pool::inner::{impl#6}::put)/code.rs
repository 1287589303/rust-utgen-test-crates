pub(super) fn put(this: PoolGuard<'_, T, F>) {
            // Since this is effectively consuming the guard and putting the
            // value back into the pool, there's no reason to run its Drop
            // impl after doing this. I don't believe there is a correctness
            // problem with doing so, but there's definitely a perf problem
            // by redoing this work. So we avoid it.
            let mut this = core::mem::ManuallyDrop::new(this);
            this.put_imp();
        }