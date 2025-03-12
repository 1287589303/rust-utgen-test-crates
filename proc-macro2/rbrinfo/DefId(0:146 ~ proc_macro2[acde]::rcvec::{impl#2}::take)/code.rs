pub(crate) fn take(self) -> RcVecBuilder<T> {
        let vec = mem::take(self.inner);
        RcVecBuilder { inner: vec }
    }