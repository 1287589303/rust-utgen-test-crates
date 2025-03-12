pub fn static_captures_len(&self) -> Option<usize> {
        self.pikevm
            .nfa()
            .static_explicit_captures_len()
            .map(|len| len.saturating_add(1))
    }