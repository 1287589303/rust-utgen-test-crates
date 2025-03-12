fn set_has_pattern_ids(&mut self) {
        self.0[0] |= 1 << 1;
    }