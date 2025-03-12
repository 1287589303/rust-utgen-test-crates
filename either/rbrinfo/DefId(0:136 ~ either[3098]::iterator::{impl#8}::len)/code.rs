fn len(&self) -> usize {
        for_both!(self.inner, ref inner => inner.len())
    }