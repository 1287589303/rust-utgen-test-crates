pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a T, E>
    where
        F: FnOnce() -> Result<&'a T, E>,
    {
        let mut ptr = self.inner.load(Ordering::Acquire);

        if ptr.is_null() {
            // TODO replace with `cast_mut` when MSRV reaches 1.65.0 (also in `set`)
            ptr = f()? as *const T as *mut T;
            let exchange = self.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            if let Err(old) = exchange {
                ptr = old;
            }
        }

        Ok(unsafe { &*ptr })
    }