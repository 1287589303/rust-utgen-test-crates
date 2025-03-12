fn canonicalize(&mut self) {
        self.ids.borrow_mut().sort();
        self.ids.borrow_mut().dedup();
    }