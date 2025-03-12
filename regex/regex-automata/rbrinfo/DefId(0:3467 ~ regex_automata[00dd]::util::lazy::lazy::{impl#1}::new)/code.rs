pub(super) const fn new(create: F) -> Lazy<T, F> {
            Lazy {
                data: AtomicPtr::new(core::ptr::null_mut()),
                create,
                owned: PhantomData,
            }
        }