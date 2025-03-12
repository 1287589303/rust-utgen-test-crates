fn from(val: Either<L, R>) -> Self {
        match val {
            Left(l) => Err(l),
            Right(r) => Ok(r),
        }
    }