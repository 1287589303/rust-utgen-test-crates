fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TryHalfMatchesIter")
            .field("it", &self.it)
            .field("finder", &"<closure>")
            .finish()
    }