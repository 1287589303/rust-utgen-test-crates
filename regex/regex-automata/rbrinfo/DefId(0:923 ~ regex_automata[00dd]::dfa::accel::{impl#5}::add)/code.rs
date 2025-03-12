pub fn add(&mut self, byte: u8) -> bool {
        if self.len() >= 3 {
            return false;
        }
        // As a special case, we totally reject trying to accelerate a state
        // with an ASCII space. In most cases, it occurs very frequently, and
        // tends to result in worse overall performance.
        if byte == b' ' {
            return false;
        }
        assert!(
            !self.contains(byte),
            "accelerator already contains {:?}",
            crate::util::escape::DebugByte(byte)
        );
        self.bytes[self.len() + 1] = byte;
        self.bytes[0] += 1;
        true
    }