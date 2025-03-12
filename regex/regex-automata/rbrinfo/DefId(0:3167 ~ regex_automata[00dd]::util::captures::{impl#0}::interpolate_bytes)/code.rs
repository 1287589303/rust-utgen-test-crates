pub fn interpolate_bytes(
        &self,
        haystack: &[u8],
        replacement: &[u8],
    ) -> Vec<u8> {
        let mut dst = vec![];
        self.interpolate_bytes_into(haystack, replacement, &mut dst);
        dst
    }