pub(crate) fn get_mut(&mut self) -> Option<RcVecMut<T>> {
        let inner = Rc::get_mut(&mut self.inner)?;
        Some(RcVecMut { inner })
    }