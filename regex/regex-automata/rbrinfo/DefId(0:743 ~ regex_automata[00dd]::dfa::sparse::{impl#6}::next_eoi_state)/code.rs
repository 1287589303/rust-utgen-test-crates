fn next_eoi_state(&self, current: StateID) -> StateID {
        self.tt.state(current).next_eoi()
    }