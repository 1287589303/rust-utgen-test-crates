pub(crate) fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
        self.inner.extend(iter);
    }