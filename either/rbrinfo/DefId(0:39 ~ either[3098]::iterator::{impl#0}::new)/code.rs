pub(crate) fn new(inner: Either<L, R>) -> Self {
        IterEither { inner }
    }