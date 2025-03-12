fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(map_either!(self.inner, ref mut inner => inner.nth_back(n)?))
    }