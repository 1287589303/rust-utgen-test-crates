pub(crate) fn get(self) -> usize {
        self.0.get().wrapping_sub(1)
    }