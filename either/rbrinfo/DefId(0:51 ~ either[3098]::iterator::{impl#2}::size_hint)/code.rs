fn size_hint(&self) -> (usize, Option<usize>) {
        for_both!(self, inner => inner.size_hint())
    }