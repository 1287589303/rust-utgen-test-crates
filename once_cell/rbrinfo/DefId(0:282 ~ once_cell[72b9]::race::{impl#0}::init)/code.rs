fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E> {
        let mut val = f()?.get();
        let exchange = self.inner.compare_exchange(0, val, Ordering::AcqRel, Ordering::Acquire);
        if let Err(old) = exchange {
            val = old;
        }
        Ok(unsafe { NonZeroUsize::new_unchecked(val) })
    }