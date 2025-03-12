pub fn alphabet_len(&self) -> usize {
        // Add one since the number of equivalence classes is one bigger than
        // the last one. But add another to account for the final EOI class
        // that isn't explicitly represented.
        usize::from(self.0[255]) + 1 + 1
    }