pub(crate) fn as_usize_untagged(&self) -> usize {
        self.as_usize_unchecked() & LazyStateID::MAX
    }