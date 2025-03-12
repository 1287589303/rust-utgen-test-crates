fn is_ascii(&self) -> bool {
        self.is_valid() && self.end <= 0x7f
    }