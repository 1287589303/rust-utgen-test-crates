pub(super) fn get(&self) -> PoolGuard<'_, T, F> {
            // Our fast path checks if the caller is the thread that "owns"
            // this pool. Or stated differently, whether it is the first thread
            // that tried to extract a value from the pool. If it is, then we
            // can return a T to the caller without going through a mutex.
            //
            // SAFETY: We must guarantee that only one thread gets access
            // to this value. Since a thread is uniquely identified by the
            // THREAD_ID thread local, it follows that if the caller's thread
            // ID is equal to the owner, then only one thread may receive this
            // value. This is also why we can get away with what looks like a
            // racy load and a store. We know that if 'owner == caller', then
            // only one thread can be here, so we don't need to worry about any
            // other thread setting the owner to something else.
            let caller = THREAD_ID.with(|id| *id);
            let owner = self.owner.load(Ordering::Acquire);
            if caller == owner {
                // N.B. We could also do a CAS here instead of a load/store,
                // but ad hoc benchmarking suggests it is slower. And a lot
                // slower in the case where `get_slow` is common.
                self.owner.store(THREAD_ID_INUSE, Ordering::Release);
                return self.guard_owned(caller);
            }
            self.get_slow(caller, owner)
        }