pub(super) fn get(&self) -> &T {
            if let Some(data) = self.poll() {
                return data;
            }
            let data = (self.create)();
            let mut ptr = Box::into_raw(Box::new(data));
            // We attempt to stuff our initialized value into our atomic
            // pointer. Upon success, we don't need to do anything. But if
            // someone else beat us to the punch, then we need to make sure
            // our newly created value is dropped.
            let result = self.data.compare_exchange(
                core::ptr::null_mut(),
                ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            if let Err(old) = result {
                // SAFETY: We created 'ptr' via Box::into_raw above, so turning
                // it back into a Box via from_raw is safe.
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
            // SAFETY: We just set the pointer above to a non-null value, even
            // in the error case, and set it to a fully initialized value
            // returned by 'create'.
            unsafe { &*ptr }
        }