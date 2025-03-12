fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.limit))
    }