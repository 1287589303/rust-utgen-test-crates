fn drop(&mut self) {
            let ptr = *self.data.get_mut();
            if !ptr.is_null() {
                // SAFETY: We just checked that 'ptr' is not null. And since
                // we have exclusive access, there are no races to worry about.
                drop(unsafe { Box::from_raw(ptr) });
            }
        }