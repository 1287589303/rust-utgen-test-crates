pub fn cross_reverse(&mut self, other: &mut Seq) {
        let (lits1, lits2) = match self.cross_preamble(other) {
            None => return,
            Some((lits1, lits2)) => (lits1, lits2),
        };
        // We basically proceed as we do in 'cross_forward' at this point,
        // except that the outer loop is now 'other' and the inner loop is now
        // 'self'. That's because 'self' corresponds to suffixes and 'other'
        // corresponds to the sequence we want to *prepend* to the suffixes.
        let newcap = lits1.len().saturating_mul(lits2.len());
        let selflits = mem::replace(lits1, Vec::with_capacity(newcap));
        for (i, otherlit) in lits2.drain(..).enumerate() {
            for selflit in selflits.iter() {
                if !selflit.is_exact() {
                    // If the suffix isn't exact, then we can't prepend
                    // anything to it. However, we still want to keep it. But
                    // we only want to keep one of them, to avoid duplication.
                    // (The duplication is okay from a correctness perspective,
                    // but wasteful.)
                    if i == 0 {
                        lits1.push(selflit.clone());
                    }
                    continue;
                }
                let mut newlit = Literal::exact(Vec::with_capacity(
                    otherlit.len() + selflit.len(),
                ));
                newlit.extend(&otherlit);
                newlit.extend(&selflit);
                if !otherlit.is_exact() {
                    newlit.make_inexact();
                }
                lits1.push(newlit);
            }
        }
        self.dedup();
    }