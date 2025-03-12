pub fn len(&self) -> Option<usize> {
        self.literals.as_ref().map(|lits| lits.len())
    }