fn is_empty(&self) -> bool {
        self.stream.is_empty() && self.extra.is_empty()
    }