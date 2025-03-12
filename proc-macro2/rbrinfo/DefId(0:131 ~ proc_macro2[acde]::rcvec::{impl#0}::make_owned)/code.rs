pub(crate) fn make_owned(mut self) -> RcVecBuilder<T>
    where
        T: Clone,
    {
        let vec = if let Some(owned) = Rc::get_mut(&mut self.inner) {
            mem::take(owned)
        } else {
            Vec::clone(&self.inner)
        };
        RcVecBuilder { inner: vec }
    }