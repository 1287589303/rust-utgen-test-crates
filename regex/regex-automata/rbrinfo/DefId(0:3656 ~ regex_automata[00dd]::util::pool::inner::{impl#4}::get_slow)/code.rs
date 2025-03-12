fn get_slow(
            &self,
            caller: usize,
            owner: usize,
        ) -> PoolGuard<'_, T, F> {
            if owner == THREAD_ID_UNOWNED {
                // This sentinel means this pool is not yet owned. We try to
                // atomically set the owner. If we do, then this thread becomes
                // the owner and we can return a guard that represents the
                // special T for the owner.
                //
                // Note that we set the owner to a different sentinel that
                // indicates that the owned value is in use. The owner ID will
                // get updated to the actual ID of this thread once the guard
                // returned by this function is put back into the pool.
                let res = self.owner.compare_exchange(
                    THREAD_ID_UNOWNED,
                    THREAD_ID_INUSE,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
                if res.is_ok() {
                    // SAFETY: A successful CAS above implies this thread is
                    // the owner and that this is the only such thread that
                    // can reach here. Thus, there is no data race.
                    unsafe {
                        *self.owner_val.get() = Some((self.create)());
                    }
                    return self.guard_owned(caller);
                }
            }
            let stack_id = caller % self.stacks.len();
            // We try to acquire exclusive access to this thread's stack, and
            // if so, grab a value from it if we can. We put this in a loop so
            // that it's easy to tweak and experiment with a different number
            // of tries. In the end, I couldn't see anything obviously better
            // than one attempt in ad hoc testing.
            for _ in 0..1 {
                let mut stack = match self.stacks[stack_id].0.try_lock() {
                    Err(_) => continue,
                    Ok(stack) => stack,
                };
                if let Some(value) = stack.pop() {
                    return self.guard_stack(value);
                }
                // Unlock the mutex guarding the stack before creating a fresh
                // value since we no longer need the stack.
                drop(stack);
                let value = Box::new((self.create)());
                return self.guard_stack(value);
            }
            // We're only here if we could get access to our stack, so just
            // create a new value. This seems like it could be wasteful, but
            // waiting for exclusive access to a stack when there's high
            // contention is brutal for perf.
            self.guard_stack_transient(Box::new((self.create)()))
        }