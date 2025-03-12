pub fn needles(&self, i: usize) -> &[u8] {
        if i >= self.len() {
            panic!("invalid accelerator index {}", i);
        }
        let bytes = self.as_bytes();
        let offset = ACCEL_TY_SIZE + i * ACCEL_CAP;
        let len = usize::from(bytes[offset]);
        &bytes[offset + 1..offset + 1 + len]
    }