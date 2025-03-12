fn is_from_word(&self) -> bool {
        self.0[0] & (1 << 2) > 0
    }