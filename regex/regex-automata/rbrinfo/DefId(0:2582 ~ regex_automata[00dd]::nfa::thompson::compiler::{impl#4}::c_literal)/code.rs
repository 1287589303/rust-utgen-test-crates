fn c_literal(&self, bytes: &[u8]) -> Result<ThompsonRef, BuildError> {
        self.c_concat(bytes.iter().copied().map(|b| self.c_range(b, b)))
    }