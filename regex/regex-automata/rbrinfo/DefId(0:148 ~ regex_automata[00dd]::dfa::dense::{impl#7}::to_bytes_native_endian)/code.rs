pub fn to_bytes_native_endian(&self) -> (Vec<u8>, usize) {
        self.to_bytes::<wire::NE>()
    }