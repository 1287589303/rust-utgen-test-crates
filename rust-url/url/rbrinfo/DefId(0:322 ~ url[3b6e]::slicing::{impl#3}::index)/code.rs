fn index(&self, range: Range<Position>) -> &str {
        &self.serialization[self.index(range.start)..self.index(range.end)]
    }