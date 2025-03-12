pub fn set(&mut self, byte: u8, class: u8) {
        self.0[usize::from(byte)] = class;
    }