fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        for_both!(self, inner => inner.any(f))
    }