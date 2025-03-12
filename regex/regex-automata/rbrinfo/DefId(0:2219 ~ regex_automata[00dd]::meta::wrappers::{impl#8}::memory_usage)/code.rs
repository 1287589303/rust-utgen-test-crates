pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "dfa-onepass")]
        {
            self.0.as_ref().map_or(0, |c| c.memory_usage())
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            0
        }
    }