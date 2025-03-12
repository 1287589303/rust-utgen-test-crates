pub(crate) fn get(&self, byte: u8) -> Start {
        self.map[usize::from(byte)]
    }