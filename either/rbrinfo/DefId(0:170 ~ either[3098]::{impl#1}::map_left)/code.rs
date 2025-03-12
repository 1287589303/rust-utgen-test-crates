pub fn map_left<F, M>(self, f: F) -> Either<M, R>
    where
        F: FnOnce(L) -> M,
    {
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(r),
        }
    }