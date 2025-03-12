pub fn factor_err(self) -> Result<Either<L, R>, E> {
        match self {
            Left(l) => l.map(Either::Left),
            Right(r) => r.map(Either::Right),
        }
    }