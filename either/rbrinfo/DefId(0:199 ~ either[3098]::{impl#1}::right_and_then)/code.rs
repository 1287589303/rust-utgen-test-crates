pub fn right_and_then<F, S>(self, f: F) -> Either<L, S>
    where
        F: FnOnce(R) -> Either<L, S>,
    {
        match self {
            Left(l) => Left(l),
            Right(r) => f(r),
        }
    }