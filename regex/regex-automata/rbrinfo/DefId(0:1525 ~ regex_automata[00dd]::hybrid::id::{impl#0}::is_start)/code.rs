pub const fn is_start(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_START > 0
    }