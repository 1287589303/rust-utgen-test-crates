fn index(&self, range: RangeTo<Position>) -> &str {
        &self.serialization[..self.index(range.end)]
    }