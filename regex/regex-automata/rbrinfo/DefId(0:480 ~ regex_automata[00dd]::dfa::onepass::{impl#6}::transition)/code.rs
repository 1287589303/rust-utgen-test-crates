fn transition(&self, sid: StateID, byte: u8) -> Transition {
        let offset = sid.as_usize() << self.stride2();
        let class = self.classes.get(byte).as_usize();
        self.table[offset + class]
    }