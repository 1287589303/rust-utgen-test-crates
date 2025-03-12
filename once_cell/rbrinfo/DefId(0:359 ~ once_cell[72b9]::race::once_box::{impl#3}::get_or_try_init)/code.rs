pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<Box<T>, E>,
        {
            let mut ptr = self.inner.load(Ordering::Acquire);

            if ptr.is_null() {
                let val = f()?;
                ptr = Box::into_raw(val);
                let exchange = self.inner.compare_exchange(
                    ptr::null_mut(),
                    ptr,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
                if let Err(old) = exchange {
                    drop(unsafe { Box::from_raw(ptr) });
                    ptr = old;
                }
            };
            Ok(unsafe { &*ptr })
        }