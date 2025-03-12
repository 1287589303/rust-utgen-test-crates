pub const fn is_dead(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_DEAD > 0
    }