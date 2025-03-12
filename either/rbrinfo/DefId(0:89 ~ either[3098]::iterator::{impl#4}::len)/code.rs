fn len(&self) -> usize {
        for_both!(self, inner => inner.len())
    }