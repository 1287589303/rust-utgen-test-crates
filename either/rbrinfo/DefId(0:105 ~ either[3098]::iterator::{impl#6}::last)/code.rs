fn last(self) -> Option<Self::Item> {
        Some(map_either!(self.inner, inner => inner.last()?))
    }