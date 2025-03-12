pub fn with_value(value: Box<T>) -> Self {
            OnceBox { inner: AtomicPtr::new(Box::into_raw(value)), ghost: PhantomData }
        }