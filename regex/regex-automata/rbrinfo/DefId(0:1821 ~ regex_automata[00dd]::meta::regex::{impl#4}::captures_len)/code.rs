pub fn captures_len(&self) -> usize {
        self.imp
            .info
            .props_union()
            .explicit_captures_len()
            .saturating_add(self.pattern_len())
    }