fn add(&mut self, id: StateID) {
        self.ids.borrow_mut().push(id);
    }