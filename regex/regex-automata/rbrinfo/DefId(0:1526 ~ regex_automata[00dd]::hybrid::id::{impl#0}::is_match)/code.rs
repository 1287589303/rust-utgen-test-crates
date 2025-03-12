pub const fn is_match(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_MATCH > 0
    }