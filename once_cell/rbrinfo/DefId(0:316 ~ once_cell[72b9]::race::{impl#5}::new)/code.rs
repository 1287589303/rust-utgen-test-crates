pub const fn new() -> OnceRef<'a, T> {
        OnceRef { inner: AtomicPtr::new(ptr::null_mut()), ghost: PhantomData }
    }