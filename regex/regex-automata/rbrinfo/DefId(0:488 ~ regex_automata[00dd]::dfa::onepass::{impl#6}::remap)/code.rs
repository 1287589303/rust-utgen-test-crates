pub(super) fn remap(&mut self, map: impl Fn(StateID) -> StateID) {
        for i in 0..self.state_len() {
            let offset = i << self.stride2();
            for b in 0..self.alphabet_len() {
                let next = self.table[offset + b].state_id();
                self.table[offset + b].set_state_id(map(next));
            }
        }
        for i in 0..self.starts.len() {
            self.starts[i] = map(self.starts[i]);
        }
    }