fn find_waiting(&self, set: &StateSet) -> Option<usize> {
        self.waiting.iter().position(|s| s == set)
    }