pub const fn new(create: F) -> Lazy<T, F> {
        Lazy(lazy::Lazy::new(create))
    }