pub fn factor_ok(self) -> Result<T, Either<L, R>> {
        match self {
            Left(l) => l.map_err(Either::Left),
            Right(r) => r.map_err(Either::Right),
        }
    }