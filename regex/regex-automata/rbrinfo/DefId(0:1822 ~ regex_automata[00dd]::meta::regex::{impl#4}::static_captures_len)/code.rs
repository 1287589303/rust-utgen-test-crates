pub fn static_captures_len(&self) -> Option<usize> {
        self.imp
            .info
            .props_union()
            .static_explicit_captures_len()
            .map(|len| len.saturating_add(1))
    }