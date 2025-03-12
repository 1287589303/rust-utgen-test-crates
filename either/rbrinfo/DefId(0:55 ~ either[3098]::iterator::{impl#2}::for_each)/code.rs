fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        for_both!(self, inner => inner.for_each(f))
    }