fn sparse_transitions(&self, sid: StateID) -> SparseTransitionIter<'_> {
        let start = sid.as_usize() << self.stride2();
        let end = start + self.alphabet_len();
        SparseTransitionIter {
            it: self.table[start..end].iter().enumerate(),
            cur: None,
        }
    }