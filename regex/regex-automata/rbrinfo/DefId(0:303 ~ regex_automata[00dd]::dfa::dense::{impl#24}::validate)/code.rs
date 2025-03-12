fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        if self.len() != dfa.special.match_len(dfa.stride()) {
            return Err(DeserializeError::generic(
                "match state length mismatch",
            ));
        }
        for si in 0..self.len() {
            let start = self.slices()[si * 2].as_usize();
            let len = self.slices()[si * 2 + 1].as_usize();
            if start >= self.pattern_ids().len() {
                return Err(DeserializeError::generic(
                    "invalid pattern ID start offset",
                ));
            }
            if start + len > self.pattern_ids().len() {
                return Err(DeserializeError::generic(
                    "invalid pattern ID length",
                ));
            }
            for mi in 0..len {
                let pid = self.pattern_id(si, mi);
                if pid.as_usize() >= self.pattern_len {
                    return Err(DeserializeError::generic(
                        "invalid pattern ID",
                    ));
                }
            }
        }
        Ok(())
    }