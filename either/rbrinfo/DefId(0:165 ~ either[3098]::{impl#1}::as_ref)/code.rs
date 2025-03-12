pub fn as_ref(&self) -> Either<&L, &R> {
        map_either!(self, inner => inner)
    }