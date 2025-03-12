fn set_transition(&mut self, sid: StateID, byte: u8, to: Transition) {
        let offset = sid.as_usize() << self.stride2();
        let class = self.classes.get(byte).as_usize();
        self.table[offset + class] = to;
    }