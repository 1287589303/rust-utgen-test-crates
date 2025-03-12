pub fn to_bytes_little_endian(&self) -> (Vec<u8>, usize) {
        self.to_bytes::<wire::LE>()
    }