pub fn either_into<T>(self) -> T
    where
        L: Into<T>,
        R: Into<T>,
    {
        for_both!(self, inner => inner.into())
    }