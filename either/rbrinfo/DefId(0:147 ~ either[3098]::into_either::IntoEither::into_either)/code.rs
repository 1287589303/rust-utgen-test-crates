fn into_either(self, into_left: bool) -> Either<Self, Self> {
        if into_left {
            Left(self)
        } else {
            Right(self)
        }
    }