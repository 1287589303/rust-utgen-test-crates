pub const fn is_unknown(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_UNKNOWN > 0
    }