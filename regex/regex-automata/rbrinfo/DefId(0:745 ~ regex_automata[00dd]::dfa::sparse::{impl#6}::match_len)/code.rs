fn match_len(&self, id: StateID) -> usize {
        self.tt.state(id).pattern_len()
    }