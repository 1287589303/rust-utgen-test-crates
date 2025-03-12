fn last(self) -> Option<Self::Item> {
        for_both!(self, inner => inner.last())
    }