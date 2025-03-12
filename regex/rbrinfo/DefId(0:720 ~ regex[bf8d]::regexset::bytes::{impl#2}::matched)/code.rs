pub fn matched(&self, index: usize) -> bool {
        self.0.contains(PatternID::new_unchecked(index))
    }