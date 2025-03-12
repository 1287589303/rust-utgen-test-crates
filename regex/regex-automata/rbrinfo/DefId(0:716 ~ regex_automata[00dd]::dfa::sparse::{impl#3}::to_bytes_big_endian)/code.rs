pub fn to_bytes_big_endian(&self) -> Vec<u8> {
        self.to_bytes::<wire::BE>()
    }