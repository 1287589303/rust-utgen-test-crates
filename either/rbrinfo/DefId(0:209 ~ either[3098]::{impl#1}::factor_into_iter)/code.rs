pub fn factor_into_iter(self) -> IterEither<L::IntoIter, R::IntoIter>
    where
        L: IntoIterator,
        R: IntoIterator,
    {
        IterEither::new(map_either!(self, inner => inner.into_iter()))
    }