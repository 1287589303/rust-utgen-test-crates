pub(crate) const fn to_dead(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_DEAD,
        )
    }