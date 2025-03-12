fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(map_either!(self.inner, ref mut inner => inner.nth(n)?))
    }