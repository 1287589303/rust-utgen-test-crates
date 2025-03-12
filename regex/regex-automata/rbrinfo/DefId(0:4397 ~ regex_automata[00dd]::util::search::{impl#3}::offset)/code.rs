pub fn offset(&self, offset: usize) -> Span {
        Span { start: self.start + offset, end: self.end + offset }
    }