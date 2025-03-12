pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {
        StateSparseTransitionIter { dense: self.transitions(), cur: None }
    }