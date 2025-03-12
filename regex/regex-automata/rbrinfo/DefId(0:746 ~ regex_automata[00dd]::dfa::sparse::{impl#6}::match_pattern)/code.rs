fn match_pattern(&self, id: StateID, match_index: usize) -> PatternID {
        // This is an optimization for the very common case of a DFA with a
        // single pattern. This conditional avoids a somewhat more costly path
        // that finds the pattern ID from the state machine, which requires
        // a bit of slicing/pointer-chasing. This optimization tends to only
        // matter when matches are frequent.
        if self.tt.pattern_len == 1 {
            return PatternID::ZERO;
        }
        self.tt.state(id).pattern_id(match_index)
    }