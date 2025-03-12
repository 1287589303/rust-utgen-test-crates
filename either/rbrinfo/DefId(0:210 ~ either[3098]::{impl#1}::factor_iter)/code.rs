pub fn factor_iter(
        &self,
    ) -> IterEither<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>
    where
        for<'a> &'a L: IntoIterator,
        for<'a> &'a R: IntoIterator,
    {
        IterEither::new(map_either!(self, inner => inner.into_iter()))
    }