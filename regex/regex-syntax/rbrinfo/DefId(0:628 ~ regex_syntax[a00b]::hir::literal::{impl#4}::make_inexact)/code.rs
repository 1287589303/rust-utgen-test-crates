pub fn make_inexact(&mut self) {
        let lits = match self.literals {
            None => return,
            Some(ref mut lits) => lits,
        };
        for lit in lits.iter_mut() {
            lit.make_inexact();
        }
    }