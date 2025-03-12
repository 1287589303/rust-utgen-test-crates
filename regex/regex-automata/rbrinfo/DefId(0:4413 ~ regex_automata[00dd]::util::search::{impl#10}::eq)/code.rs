fn eq(&self, range: &Range<usize>) -> bool {
        self.start == range.start && self.end == range.end
    }