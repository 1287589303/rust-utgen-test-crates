pub fn minimize_by_preference(&mut self) {
        if let Some(ref mut lits) = self.literals {
            PreferenceTrie::minimize(lits, false);
        }
    }