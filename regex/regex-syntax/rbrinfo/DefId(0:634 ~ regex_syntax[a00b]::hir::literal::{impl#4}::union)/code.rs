pub fn union(&mut self, other: &mut Seq) {
        let lits2 = match other.literals {
            None => {
                // Unioning with an infinite sequence always results in an
                // infinite sequence.
                self.make_infinite();
                return;
            }
            Some(ref mut lits) => lits.drain(..),
        };
        let lits1 = match self.literals {
            None => return,
            Some(ref mut lits) => lits,
        };
        lits1.extend(lits2);
        self.dedup();
    }