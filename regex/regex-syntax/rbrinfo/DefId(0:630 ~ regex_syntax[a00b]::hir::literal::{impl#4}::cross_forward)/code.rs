pub fn cross_forward(&mut self, other: &mut Seq) {
        let (lits1, lits2) = match self.cross_preamble(other) {
            None => return,
            Some((lits1, lits2)) => (lits1, lits2),
        };
        let newcap = lits1.len().saturating_mul(lits2.len());
        for selflit in mem::replace(lits1, Vec::with_capacity(newcap)) {
            if !selflit.is_exact() {
                lits1.push(selflit);
                continue;
            }
            for otherlit in lits2.iter() {
                let mut newlit = Literal::exact(Vec::with_capacity(
                    selflit.len() + otherlit.len(),
                ));
                newlit.extend(&selflit);
                newlit.extend(&otherlit);
                if !otherlit.is_exact() {
                    newlit.make_inexact();
                }
                lits1.push(newlit);
            }
        }
        lits2.drain(..);
        self.dedup();
    }