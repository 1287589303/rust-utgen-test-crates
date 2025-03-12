fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        wrap_either!(self.inner => .for_each(f))
    }