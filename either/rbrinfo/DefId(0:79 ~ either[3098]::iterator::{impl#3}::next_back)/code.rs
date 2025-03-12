fn next_back(&mut self) -> Option<Self::Item> {
        for_both!(self, inner => inner.next_back())
    }