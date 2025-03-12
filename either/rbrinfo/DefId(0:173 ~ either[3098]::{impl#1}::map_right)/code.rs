pub fn map_right<F, S>(self, f: F) -> Either<L, S>
    where
        F: FnOnce(R) -> S,
    {
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }