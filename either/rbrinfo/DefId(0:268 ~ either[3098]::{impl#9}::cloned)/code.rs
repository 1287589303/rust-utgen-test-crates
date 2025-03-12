pub fn cloned(self) -> Either<L, R>
    where
        L: Clone,
        R: Clone,
    {
        map_either!(self, inner => inner.clone())
    }