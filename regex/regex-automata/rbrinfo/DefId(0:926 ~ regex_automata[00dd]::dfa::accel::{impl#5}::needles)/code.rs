fn needles(&self) -> &[u8] {
        &self.bytes[1..1 + self.len()]
    }