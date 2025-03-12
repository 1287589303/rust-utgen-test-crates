pub fn get(&self, byte: u8) -> u8 {
        self.0[usize::from(byte)]
    }