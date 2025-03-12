fn memory_usage(&self) -> usize {
        (self.slices().len() + self.pattern_ids().len()) * PatternID::SIZE
    }