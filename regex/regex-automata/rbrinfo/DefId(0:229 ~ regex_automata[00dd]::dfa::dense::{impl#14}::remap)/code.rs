fn remap(&mut self, id: StateID, map: impl Fn(StateID) -> StateID) {
        for byte in 0..self.alphabet_len() {
            let i = id.as_usize() + byte;
            let next = self.table()[i];
            self.table_mut()[id.as_usize() + byte] = map(next);
        }
    }