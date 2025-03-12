pub fn map<F, M>(self, f: F) -> Either<M, M>
    where
        F: FnOnce(T) -> M,
    {
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(f(r)),
        }
    }