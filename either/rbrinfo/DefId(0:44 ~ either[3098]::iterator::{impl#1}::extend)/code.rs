fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>,
    {
        for_both!(self, inner => inner.extend(iter))
    }