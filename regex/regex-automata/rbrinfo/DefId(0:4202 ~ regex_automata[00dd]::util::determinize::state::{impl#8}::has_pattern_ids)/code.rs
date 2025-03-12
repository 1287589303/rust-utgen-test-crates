fn has_pattern_ids(&self) -> bool {
        self.0[0] & (1 << 1) > 0
    }