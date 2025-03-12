fn swap(&mut self, id1: StateID, id2: StateID) {
        assert!(self.is_valid(id1), "invalid 'id1' state: {:?}", id1);
        assert!(self.is_valid(id2), "invalid 'id2' state: {:?}", id2);
        // We only need to swap the parts of the state that are used. So if the
        // stride is 64, but the alphabet length is only 33, then we save a lot
        // of work.
        for b in 0..self.classes.alphabet_len() {
            self.table.swap(id1.as_usize() + b, id2.as_usize() + b);
        }
    }