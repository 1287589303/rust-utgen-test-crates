pub(crate) const fn as_usize_unchecked(&self) -> usize {
        // FIXME: Use as_usize() once const functions in traits are stable.
        self.0 as usize
    }