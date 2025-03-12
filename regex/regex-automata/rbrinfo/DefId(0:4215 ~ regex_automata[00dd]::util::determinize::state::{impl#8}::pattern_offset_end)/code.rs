fn pattern_offset_end(&self) -> usize {
        let encoded = self.encoded_pattern_len();
        if encoded == 0 {
            return 9;
        }
        // This arithmetic is OK since we were able to address this many bytes
        // when writing to the state, thus, it must fit into a usize.
        encoded.checked_mul(4).unwrap().checked_add(13).unwrap()
    }