pub fn map_either<F, G, M, S>(self, f: F, g: G) -> Either<M, S>
    where
        F: FnOnce(L) -> M,
        G: FnOnce(R) -> S,
    {
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(g(r)),
        }
    }