pub(super) fn swap_states(&mut self, id1: StateID, id2: StateID) {
        let o1 = id1.as_usize() << self.stride2();
        let o2 = id2.as_usize() << self.stride2();
        for b in 0..self.stride() {
            self.table.swap(o1 + b, o2 + b);
        }
    }