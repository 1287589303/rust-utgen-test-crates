fn collect<B>(self) -> B
    where
        B: iter::FromIterator<Self::Item>,
    {
        wrap_either!(self.inner => .collect())
    }