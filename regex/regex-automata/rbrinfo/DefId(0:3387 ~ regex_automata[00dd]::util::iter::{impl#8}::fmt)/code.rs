fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TryMatchesIter")
            .field("it", &self.it)
            .field("finder", &"<closure>")
            .finish()
    }