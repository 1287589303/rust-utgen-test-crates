pub(crate) fn make_mut(&mut self) -> RcVecMut<T>
    where
        T: Clone,
    {
        RcVecMut {
            inner: Rc::make_mut(&mut self.inner),
        }
    }