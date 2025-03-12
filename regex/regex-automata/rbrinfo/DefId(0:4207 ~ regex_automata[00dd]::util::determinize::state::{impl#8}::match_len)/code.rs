fn match_len(&self) -> usize {
        if !self.is_match() {
            return 0;
        } else if !self.has_pattern_ids() {
            1
        } else {
            self.encoded_pattern_len()
        }
    }