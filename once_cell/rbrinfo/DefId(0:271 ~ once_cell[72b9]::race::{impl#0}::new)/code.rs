pub const fn new() -> OnceNonZeroUsize {
        OnceNonZeroUsize { inner: AtomicUsize::new(0) }
    }