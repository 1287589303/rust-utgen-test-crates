fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        for_both!(self, inner => inner.nth_back(n))
    }