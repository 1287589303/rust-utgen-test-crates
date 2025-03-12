pub fn reverse_literals(&mut self) {
        if let Some(ref mut lits) = self.literals {
            for lit in lits.iter_mut() {
                lit.reverse();
            }
        }
    }