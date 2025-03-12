fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq {
        let mut seq = Seq::singleton(self::Literal::exact(vec![]));
        for hir in it {
            // If every element in the sequence is inexact, then a cross
            // product will always be a no-op. Thus, there is nothing else we
            // can add to it and can quit early. Note that this also includes
            // infinite sequences.
            if seq.is_inexact() {
                break;
            }
            // Note that 'cross' also dispatches based on whether we're
            // extracting prefixes or suffixes.
            seq = self.cross(seq, &mut self.extract(hir));
        }
        seq
    }