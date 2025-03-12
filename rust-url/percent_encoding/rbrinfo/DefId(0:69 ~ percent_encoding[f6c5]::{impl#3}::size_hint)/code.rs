fn size_hint(&self) -> (usize, Option<usize>) {
        let bytes = self.bytes.len();
        ((bytes + 2) / 3, Some(bytes))
    }