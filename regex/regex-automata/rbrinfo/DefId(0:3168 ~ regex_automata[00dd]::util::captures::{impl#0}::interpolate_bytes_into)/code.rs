pub fn interpolate_bytes_into(
        &self,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut Vec<u8>,
    ) {
        interpolate::bytes(
            replacement,
            |index, dst| {
                let span = match self.get_group(index) {
                    None => return,
                    Some(span) => span,
                };
                dst.extend_from_slice(&haystack[span]);
            },
            |name| self.group_info().to_index(self.pattern()?, name),
            dst,
        );
    }