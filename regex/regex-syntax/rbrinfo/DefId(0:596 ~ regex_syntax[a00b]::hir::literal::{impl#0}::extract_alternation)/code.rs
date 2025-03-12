fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(
        &self,
        it: I,
    ) -> Seq {
        let mut seq = Seq::empty();
        for hir in it {
            // Once our 'seq' is infinite, every subsequent union
            // operation on it will itself always result in an
            // infinite sequence. Thus, it can never change and we can
            // short-circuit.
            if !seq.is_finite() {
                break;
            }
            seq = self.union(seq, &mut self.extract(hir));
        }
        seq
    }