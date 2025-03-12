pub fn left_and_then<F, S>(self, f: F) -> Either<S, R>
    where
        F: FnOnce(L) -> Either<S, R>,
    {
        match self {
            Left(l) => f(l),
            Right(r) => Right(r),
        }
    }