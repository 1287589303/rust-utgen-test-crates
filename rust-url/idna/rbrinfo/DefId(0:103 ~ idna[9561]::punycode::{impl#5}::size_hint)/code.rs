fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len - self.position;
        (len, Some(len))
    }