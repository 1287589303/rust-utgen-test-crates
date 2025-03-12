pub(crate) fn as_mut(&mut self) -> RcVecMut<T> {
        RcVecMut {
            inner: &mut self.inner,
        }
    }