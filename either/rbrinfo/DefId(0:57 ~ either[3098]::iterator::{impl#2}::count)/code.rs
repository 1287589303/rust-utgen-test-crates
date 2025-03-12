fn count(self) -> usize {
        for_both!(self, inner => inner.count())
    }