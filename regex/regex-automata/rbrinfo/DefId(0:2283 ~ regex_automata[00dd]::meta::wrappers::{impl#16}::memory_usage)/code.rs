pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "hybrid")]
        {
            self.0.as_ref().map_or(0, |c| c.memory_usage())
        }
        #[cfg(not(feature = "hybrid"))]
        {
            0
        }
    }