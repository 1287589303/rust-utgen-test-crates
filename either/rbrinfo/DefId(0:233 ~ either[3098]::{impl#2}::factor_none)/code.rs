pub fn factor_none(self) -> Option<Either<L, R>> {
        match self {
            Left(l) => l.map(Either::Left),
            Right(r) => r.map(Either::Right),
        }
    }