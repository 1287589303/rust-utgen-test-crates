fn max_haystack_len(&self) -> usize {
        #[cfg(feature = "nfa-backtrack")]
        {
            self.0.max_haystack_len()
        }
        #[cfg(not(feature = "nfa-backtrack"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }