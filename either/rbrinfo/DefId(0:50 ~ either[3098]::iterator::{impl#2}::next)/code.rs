fn next(&mut self) -> Option<Self::Item> {
        for_both!(self, inner => inner.next())
    }