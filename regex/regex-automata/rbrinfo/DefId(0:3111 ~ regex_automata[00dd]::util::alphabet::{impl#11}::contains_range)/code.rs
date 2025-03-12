pub(crate) fn contains_range(&self, start: u8, end: u8) -> bool {
        (start..=end).all(|b| self.contains(b))
    }