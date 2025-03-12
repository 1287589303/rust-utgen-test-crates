pub(crate) fn as_mut(&mut self) -> RcVecMut<T> {
        RcVecMut { inner: self.inner }
    }