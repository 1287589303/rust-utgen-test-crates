fn poll(&self) -> Option<&T> {
            let ptr = self.data.load(Ordering::Acquire);
            if ptr.is_null() {
                return None;
            }
            // SAFETY: We just checked that the pointer is not null. Since it's
            // not null, it must have been fully initialized by 'get' at some
            // point.
            Some(unsafe { &*ptr })
        }