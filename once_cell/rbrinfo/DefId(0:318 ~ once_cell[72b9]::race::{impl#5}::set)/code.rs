pub fn set(&self, value: &'a T) -> Result<(), ()> {
        let ptr = value as *const T as *mut T;
        let exchange =
            self.inner.compare_exchange(ptr::null_mut(), ptr, Ordering::AcqRel, Ordering::Acquire);
        match exchange {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }