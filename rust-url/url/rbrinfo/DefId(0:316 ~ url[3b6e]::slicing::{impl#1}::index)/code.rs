fn index(&self, range: RangeFrom<Position>) -> &str {
        &self.serialization[self.index(range.start)..]
    }