pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "dfa-build")]
        {
            self.0.forward().memory_usage() + self.0.reverse().memory_usage()
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }