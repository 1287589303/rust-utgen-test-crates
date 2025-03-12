pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
        self.caps.interpolate_bytes_into(self.haystack, replacement, dst);
    }