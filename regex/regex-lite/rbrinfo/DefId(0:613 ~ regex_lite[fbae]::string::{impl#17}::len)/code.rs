pub fn len(&self) -> usize {
        // We always have twice as many slots as groups.
        self.0.len().checked_shr(1).unwrap()
    }