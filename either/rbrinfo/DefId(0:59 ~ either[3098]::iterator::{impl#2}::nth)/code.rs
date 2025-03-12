fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for_both!(self, inner => inner.nth(n))
    }