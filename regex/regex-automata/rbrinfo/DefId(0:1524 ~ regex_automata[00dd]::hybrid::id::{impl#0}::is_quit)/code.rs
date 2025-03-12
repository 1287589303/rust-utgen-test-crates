pub const fn is_quit(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_QUIT > 0
    }