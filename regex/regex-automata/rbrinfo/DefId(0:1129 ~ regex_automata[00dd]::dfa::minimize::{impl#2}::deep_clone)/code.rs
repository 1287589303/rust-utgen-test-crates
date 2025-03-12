fn deep_clone(&self) -> StateSet {
        let ids = self.ids.borrow().iter().cloned().collect();
        StateSet { ids: Rc::new(RefCell::new(ids)) }
    }