pub fn factor_iter_mut(
        &mut self,
    ) -> IterEither<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>
    where
        for<'a> &'a mut L: IntoIterator,
        for<'a> &'a mut R: IntoIterator,
    {
        IterEither::new(map_either!(self, inner => inner.into_iter()))
    }