fn clone(&self) -> Self {
        RcVec {
            inner: Rc::clone(&self.inner),
        }
    }