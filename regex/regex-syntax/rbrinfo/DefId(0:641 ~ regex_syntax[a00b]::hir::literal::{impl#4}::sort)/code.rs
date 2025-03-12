pub fn sort(&mut self) {
        if let Some(ref mut lits) = self.literals {
            lits.sort();
        }
    }