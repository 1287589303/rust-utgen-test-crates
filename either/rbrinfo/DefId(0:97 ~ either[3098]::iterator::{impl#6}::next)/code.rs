fn next(&mut self) -> Option<Self::Item> {
        Some(map_either!(self.inner, ref mut inner => inner.next()?))
    }