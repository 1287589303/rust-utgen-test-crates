pub(super) fn value(&self) -> &T {
            match self.value {
                Ok(ref v) => &**v,
                // SAFETY: This is safe because the only way a PoolGuard gets
                // created for self.value=Err is when the current thread
                // corresponds to the owning thread, of which there can only
                // be one. Thus, we are guaranteed to be providing exclusive
                // access here which makes this safe.
                //
                // Also, since 'owner_val' is guaranteed to be initialized
                // before an owned PoolGuard is created, the unchecked unwrap
                // is safe.
                Err(id) => unsafe {
                    // This assert is *not* necessary for safety, since we
                    // should never be here if the guard had been put back into
                    // the pool. This is a sanity check to make sure we didn't
                    // break an internal invariant.
                    debug_assert_ne!(THREAD_ID_DROPPED, id);
                    (*self.pool.owner_val.get()).as_ref().unwrap_unchecked()
                },
            }
        }