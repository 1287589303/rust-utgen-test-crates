pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }