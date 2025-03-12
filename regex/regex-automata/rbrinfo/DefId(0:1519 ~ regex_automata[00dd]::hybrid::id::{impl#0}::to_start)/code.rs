pub(crate) const fn to_start(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_START,
        )
    }