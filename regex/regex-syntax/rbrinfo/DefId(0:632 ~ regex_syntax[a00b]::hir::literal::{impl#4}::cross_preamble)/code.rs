fn cross_preamble<'a>(
        &'a mut self,
        other: &'a mut Seq,
    ) -> Option<(&'a mut Vec<Literal>, &'a mut Vec<Literal>)> {
        let lits2 = match other.literals {
            None => {
                // If our current seq contains the empty string and the seq
                // we're adding matches any literal, then it follows that the
                // current seq must now also match any literal.
                //
                // Otherwise, we just have to make sure everything in this
                // sequence is inexact.
                if self.min_literal_len() == Some(0) {
                    *self = Seq::infinite();
                } else {
                    self.make_inexact();
                }
                return None;
            }
            Some(ref mut lits) => lits,
        };
        let lits1 = match self.literals {
            None => {
                // If we aren't going to make it to the end of this routine
                // where lits2 is drained, then we need to do it now.
                lits2.drain(..);
                return None;
            }
            Some(ref mut lits) => lits,
        };
        Some((lits1, lits2))
    }