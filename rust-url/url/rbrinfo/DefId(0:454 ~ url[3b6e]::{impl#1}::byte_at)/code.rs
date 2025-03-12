fn byte_at(&self, i: u32) -> u8 {
        self.serialization.as_bytes()[i as usize]
    }