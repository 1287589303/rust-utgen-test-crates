fn put_imp(&mut self) {
            match core::mem::replace(&mut self.value, Err(THREAD_ID_DROPPED)) {
                Ok(value) => {
                    // If we were told to discard this value then don't bother
                    // trying to put it back into the pool. This occurs when
                    // the pop operation failed to acquire a lock and we
                    // decided to create a new value in lieu of contending for
                    // the lock.
                    if self.discard {
                        return;
                    }
                    self.pool.put_value(value);
                }
                // If this guard has a value "owned" by the thread, then
                // the Pool guarantees that this is the ONLY such guard.
                // Therefore, in order to place it back into the pool and make
                // it available, we need to change the owner back to the owning
                // thread's ID. But note that we use the ID that was stored in
                // the guard, since a guard can be moved to another thread and
                // dropped. (A previous iteration of this code read from the
                // THREAD_ID thread local, which uses the ID of the current
                // thread which may not be the ID of the owning thread! This
                // also avoids the TLS access, which is likely a hair faster.)
                Err(owner) => {
                    // If we hit this point, it implies 'put_imp' has been
                    // called multiple times for the same guard which in turn
                    // corresponds to a bug in this implementation.
                    assert_ne!(THREAD_ID_DROPPED, owner);
                    self.pool.owner.store(owner, Ordering::Release);
                }
            }
        }