fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        let tt = &dfa.tt;
        if !self.universal_start_unanchored.map_or(true, |s| tt.is_valid(s)) {
            return Err(DeserializeError::generic(
                "found invalid universal unanchored starting state ID",
            ));
        }
        if !self.universal_start_anchored.map_or(true, |s| tt.is_valid(s)) {
            return Err(DeserializeError::generic(
                "found invalid universal anchored starting state ID",
            ));
        }
        for &id in self.table() {
            if !tt.is_valid(id) {
                return Err(DeserializeError::generic(
                    "found invalid starting state ID",
                ));
            }
        }
        Ok(())
    }