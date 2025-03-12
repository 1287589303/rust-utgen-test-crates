pub fn new(create: F) -> Pool<T, F> {
        Pool(alloc::boxed::Box::new(inner::Pool::new(create)))
    }