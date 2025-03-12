pub const fn is_tagged(&self) -> bool {
        self.as_usize_unchecked() > LazyStateID::MAX
    }