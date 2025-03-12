fn clone(&self) -> Self {
        match self {
            Left(inner) => Left(inner.clone()),
            Right(inner) => Right(inner.clone()),
        }
    }