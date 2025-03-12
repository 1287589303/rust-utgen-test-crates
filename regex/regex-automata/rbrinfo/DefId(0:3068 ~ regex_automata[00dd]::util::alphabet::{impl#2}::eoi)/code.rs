pub fn eoi(&self) -> Unit {
        // The alphabet length already includes the EOI sentinel, hence why
        // we subtract 1.
        Unit::eoi(self.alphabet_len().checked_sub(1).unwrap())
    }