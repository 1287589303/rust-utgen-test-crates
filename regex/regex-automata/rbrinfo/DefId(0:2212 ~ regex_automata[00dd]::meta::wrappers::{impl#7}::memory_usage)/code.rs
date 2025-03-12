pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "dfa-onepass")]
        {
            self.0.memory_usage()
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }