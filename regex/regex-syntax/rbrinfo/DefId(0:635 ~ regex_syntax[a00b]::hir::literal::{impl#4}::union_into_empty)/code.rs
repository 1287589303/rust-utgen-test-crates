pub fn union_into_empty(&mut self, other: &mut Seq) {
        let lits2 = other.literals.as_mut().map(|lits| lits.drain(..));
        let lits1 = match self.literals {
            None => return,
            Some(ref mut lits) => lits,
        };
        let first_empty = match lits1.iter().position(|m| m.is_empty()) {
            None => return,
            Some(i) => i,
        };
        let lits2 = match lits2 {
            None => {
                // Note that we are only here if we've found an empty literal,
                // which implies that an infinite sequence infects this seq and
                // also turns it into an infinite sequence.
                self.literals = None;
                return;
            }
            Some(lits) => lits,
        };
        // Clearing out the empties needs to come before the splice because
        // the splice might add more empties that we don't want to get rid
        // of. Since we're splicing into the position of the first empty, the
        // 'first_empty' position computed above is still correct.
        lits1.retain(|m| !m.is_empty());
        lits1.splice(first_empty..first_empty, lits2);
        self.dedup();
    }