pub const fn new() -> OnceBox<T> {
            OnceBox { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
        }