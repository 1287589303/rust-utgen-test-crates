pub fn to_bytes_little_endian(&self) -> Vec<u8> {
        self.to_bytes::<wire::LE>()
    }