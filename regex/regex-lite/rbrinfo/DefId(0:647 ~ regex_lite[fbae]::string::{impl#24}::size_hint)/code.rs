fn size_hint(&self) -> (usize, Option<usize>) {
        self.splits.size_hint()
    }