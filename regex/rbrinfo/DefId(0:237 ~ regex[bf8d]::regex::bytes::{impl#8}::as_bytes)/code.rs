pub fn as_bytes(&self) -> &'h [u8] {
        &self.haystack[self.range()]
    }