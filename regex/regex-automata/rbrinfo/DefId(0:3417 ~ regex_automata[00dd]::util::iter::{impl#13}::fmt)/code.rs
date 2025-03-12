fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TryCapturesIter")
            .field("it", &self.it)
            .field("caps", &self.caps)
            .field("finder", &"<closure>")
            .finish()
    }