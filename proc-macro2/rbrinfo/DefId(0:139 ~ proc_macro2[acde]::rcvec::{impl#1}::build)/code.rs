pub(crate) fn build(self) -> RcVec<T> {
        RcVec {
            inner: Rc::new(self.inner),
        }
    }