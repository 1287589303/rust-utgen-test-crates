fn collect<B>(self) -> B
    where
        B: iter::FromIterator<Self::Item>,
    {
        for_both!(self, inner => inner.collect())
    }