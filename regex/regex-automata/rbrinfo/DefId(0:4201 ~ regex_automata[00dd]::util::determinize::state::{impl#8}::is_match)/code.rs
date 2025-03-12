fn is_match(&self) -> bool {
        self.0[0] & (1 << 0) > 0
    }